use anyhow::Result;
use chrono::{DateTime, Utc};
use crate::models::{HistoricalDataPoint, YahooFinanceResponse};

pub struct YahooFinanceClient {
    client: reqwest::Client,
}

impl YahooFinanceClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub async fn fetch_historical_data(
        &self,
        ticker: &str,
        period1: i64,
        period2: i64,
    ) -> Result<Vec<HistoricalDataPoint>> {
        let url = format!(
            "https://query1.finance.yahoo.com/v8/finance/chart/{}?period1={}&period2={}&interval=1d",
            ticker, period1, period2
        );
        
        let response = self.client
            .get(&url)
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "Yahoo Finance API error: {}",
                response.status()
            ));
        }

        let yahoo_response: YahooFinanceResponse = response.json().await?;
        
        if yahoo_response.chart.result.is_empty() {
            return Ok(Vec::new());
        }

        let result = &yahoo_response.chart.result[0];
        let timestamps = &result.timestamp;
        let quote = &result.indicators.quote[0];

        let mut data_points = Vec::new();

        for i in 0..timestamps.len() {
            if let (Some(open), Some(high), Some(low), Some(close), Some(volume)) = (
                quote.open[i],
                quote.high[i],
                quote.low[i],
                quote.close[i],
                quote.volume[i],
            ) {
                let dt = DateTime::from_timestamp(timestamps[i], 0)
                    .ok_or_else(|| anyhow::anyhow!("Invalid timestamp"))?;
                let date = dt.format("%Y-%m-%d").to_string();

                data_points.push(HistoricalDataPoint {
                    date,
                    open,
                    high,
                    low,
                    close,
                    volume,
                });
            }
        }

        Ok(data_points)
    }

    pub async fn fetch_historical_data_from(
        &self,
        ticker: &str,
        from_date: &str,
    ) -> Result<Vec<HistoricalDataPoint>> {
        let from_datetime = chrono::NaiveDate::parse_from_str(from_date, "%Y-%m-%d")?
            .and_hms_opt(0, 0, 0)
            .ok_or_else(|| anyhow::anyhow!("Invalid time"))?
            .and_utc();
        
        let period1 = from_datetime.timestamp();
        let period2 = Utc::now().timestamp();

        self.fetch_historical_data(ticker, period1, period2).await
    }

    pub async fn fetch_all_historical_data(
        &self,
        ticker: &str,
    ) -> Result<Vec<HistoricalDataPoint>> {
        let period1 = 0;
        let period2 = Utc::now().timestamp();

        self.fetch_historical_data(ticker, period1, period2).await
    }
}
