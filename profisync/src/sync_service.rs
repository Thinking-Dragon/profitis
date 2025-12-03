use anyhow::Result;
use chrono::{NaiveDate, Utc};
use std::collections::HashSet;
use std::time::Duration;
use tokio::time::{self, Interval};
use indicatif::{ProgressBar, ProgressStyle, MultiProgress};
use console::style;
use crate::models::{HistoricalDataPoint, Stock};
use crate::profiserve_client::ProfiserveClient;
use crate::yahoo_finance::YahooFinanceClient;

pub struct SyncService {
    profiserve_client: ProfiserveClient,
    yahoo_client: YahooFinanceClient,
    sync_interval: Duration,
}

impl SyncService {
    pub fn new(profiserve_url: String, sync_interval_secs: u64) -> Self {
        Self {
            profiserve_client: ProfiserveClient::new(profiserve_url),
            yahoo_client: YahooFinanceClient::new(),
            sync_interval: Duration::from_secs(sync_interval_secs),
        }
    }

    pub async fn start(&self) -> Result<()> {
        let mut interval: Interval = time::interval(self.sync_interval);

        loop {
            interval.tick().await;
            
            println!("\n{}", style("━".repeat(60)).dim());
            println!("{} {}", 
                style("🔄 Synchronizing stocks").cyan().bold(),
                style(Utc::now().format("%Y-%m-%d %H:%M:%S")).dim()
            );
            println!("{}\n", style("━".repeat(60)).dim());
            
            if let Err(e) = self.sync_all_stocks().await {
                eprintln!("{} {}", style("✗").red().bold(), style(format!("Error: {}", e)).red());
            }
        }
    }

    async fn sync_all_stocks(&self) -> Result<()> {
        let spinner: ProgressBar = ProgressBar::new_spinner();
        spinner.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.cyan} {msg}")
                .unwrap()
        );
        spinner.set_message("Fetching stocks from profiserve...");
        spinner.enable_steady_tick(Duration::from_millis(100));
        
        let stocks: Vec<Stock> = self.profiserve_client.get_stocks().await?;
        spinner.finish_and_clear();
        
        if stocks.is_empty() {
            println!("{} {}", 
                style("ℹ").blue().bold(),
                style("No stocks found in profiserve").yellow()
            );
            return Ok(());
        }

        println!("{} Found {} stocks to synchronize\n", 
            style("📊").bold(),
            style(stocks.len()).cyan().bold()
        );

        let multi: MultiProgress = MultiProgress::new();
        let overall_pb: ProgressBar = multi.add(ProgressBar::new(stocks.len() as u64));
        overall_pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} stocks {msg}")
                .unwrap()
                .progress_chars("█▓▒░  ")
        );

        for stock in &stocks {
            overall_pb.set_message(format!("Processing {}", style(&stock.ticker).cyan()));
            
            if let Err(e) = self.sync_stock(&stock, &multi).await {
                println!("{} {} - {}", 
                    style("✗").red().bold(),
                    style(&stock.ticker).red(),
                    style(format!("{}", e)).dim()
                );
            }
            
            overall_pb.inc(1);
        }

        overall_pb.finish_and_clear();
        Ok(())
    }

    async fn sync_stock(&self, stock: &Stock, multi: &MultiProgress) -> Result<()> {
        let pb: ProgressBar = multi.add(ProgressBar::new_spinner());
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("  {spinner:.cyan} {msg}")
                .unwrap()
        );
        pb.enable_steady_tick(Duration::from_millis(100));
        pb.set_message(format!("{} Checking latest data...", style(&stock.ticker).cyan().bold()));

        let latest_date: Option<String> = self.profiserve_client.get_latest_date(&stock.ticker).await?;

        let new_data_points: Vec<HistoricalDataPoint> = match latest_date {
            Some(date) => {
                pb.set_message(format!("{} Latest: {}", 
                    style(&stock.ticker).cyan().bold(),
                    style(&date).dim()
                ));
                
                let latest_naive_date: NaiveDate = NaiveDate::parse_from_str(&date, "%Y-%m-%d")?;
                let next_day: NaiveDate = latest_naive_date.succ_opt()
                    .ok_or_else(|| anyhow::anyhow!("Failed to calculate next day"))?;
                
                let today: NaiveDate = Utc::now().date_naive();
                
                if next_day >= today {
                    pb.finish_with_message(format!("{} {}", 
                        style(&stock.ticker).cyan().bold(),
                        style("✓ Up to date").green()
                    ));
                    return Ok(());
                }
                
                let from_date: String = next_day.format("%Y-%m-%d").to_string();
                pb.set_message(format!("{} Fetching from Yahoo Finance...", 
                    style(&stock.ticker).cyan().bold()
                ));
                
                self.yahoo_client.fetch_historical_data_from(&stock.ticker, &from_date).await?
            }
            None => {
                pb.set_message(format!("{} Fetching all history...", 
                    style(&stock.ticker).cyan().bold()
                ));
                
                self.yahoo_client.fetch_all_historical_data(&stock.ticker).await?
            }
        };

        if new_data_points.is_empty() {
            pb.finish_with_message(format!("{} {}", 
                style(&stock.ticker).cyan().bold(),
                style("✓ No new data").green()
            ));
            return Ok(());
        }

        pb.set_message(format!("{} Synchronizing {} quotes...", 
            style(&stock.ticker).cyan().bold(),
            style(new_data_points.len()).yellow()
        ));

        let existing_dates: HashSet<String> = self.profiserve_client.get_existing_dates(&stock.ticker).await?;
        
        let filtered_data_points: Vec<HistoricalDataPoint> = new_data_points.into_iter()
            .filter(|dp: &HistoricalDataPoint| !existing_dates.contains(&dp.date))
            .collect();

        if filtered_data_points.is_empty() {
            pb.finish_with_message(format!("{} {}", 
                style(&stock.ticker).cyan().bold(),
                style("✓ All data already exists").green()
            ));
            return Ok(());
        }

        pb.set_message(format!("{} Uploading {} new quotes...", 
            style(&stock.ticker).cyan().bold(),
            style(filtered_data_points.len()).yellow()
        ));

        let mut success_count = 0;
        for data_point in filtered_data_points {
            match self.profiserve_client.create_historical_data_point(&stock.ticker, &data_point).await {
                Ok(_) => {
                    success_count += 1;
                }
                Err(e) => {
                    pb.println(format!("    {} Failed to upload {}: {}", 
                        style("⚠").yellow(),
                        style(&data_point.date).dim(),
                        style(format!("{}", e)).dim()
                    ));
                }
            }
        }

        pb.finish_with_message(format!("{} {} {}", 
            style(&stock.ticker).cyan().bold(),
            style("✓").green().bold(),
            style(format!("Synchronized {} quotes", success_count)).green()
        ));

        Ok(())
    }
}
