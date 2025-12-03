use anyhow::Result;
use crate::models::{Stock, HistoricalDataPoint, HistoricalDataList};

pub struct ProfiserveClient {
    base_url: String,
    client: reqwest::Client,
}

impl ProfiserveClient {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: reqwest::Client::new(),
        }
    }

    pub async fn get_stocks(&self) -> Result<Vec<Stock>> {
        let url = format!("{}/api/v1/stocks", self.base_url);
        
        let response = self.client
            .get(&url)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "Failed to fetch stocks: {}",
                response.status()
            ));
        }

        let stocks: Vec<Stock> = response.json().await?;
        Ok(stocks)
    }

    pub async fn get_historical_data(&self, ticker: &str) -> Result<Option<HistoricalDataList>> {
        let url = format!("{}/api/v1/stocks/{}/history", self.base_url, ticker);
        
        let response = self.client
            .get(&url)
            .send()
            .await?;

        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Ok(None);
        }

        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "Failed to fetch historical data for {}: {}",
                ticker,
                response.status()
            ));
        }

        let data: HistoricalDataList = response.json().await?;
        Ok(Some(data))
    }

    pub async fn create_historical_data_point(
        &self,
        ticker: &str,
        data_point: &HistoricalDataPoint,
    ) -> Result<()> {
        let url = format!("{}/api/v1/stocks/{}/history", self.base_url, ticker);
        
        let response = self.client
            .post(&url)
            .json(data_point)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "Failed to create historical data for {}: {}",
                ticker,
                response.status()
            ));
        }

        Ok(())
    }

    pub async fn get_latest_date(&self, ticker: &str) -> Result<Option<String>> {
        match self.get_historical_data(ticker).await? {
            Some(data) => {
                if data.data.is_empty() {
                    Ok(None)
                } else {
                    let latest = data.data.iter()
                        .map(|dp| dp.date.as_str())
                        .max()
                        .map(String::from);
                    Ok(latest)
                }
            }
            None => Ok(None),
        }
    }

    pub async fn get_existing_dates(&self, ticker: &str) -> Result<std::collections::HashSet<String>> {
        use std::collections::HashSet;
        
        match self.get_historical_data(ticker).await? {
            Some(data) => {
                let dates: HashSet<String> = data.data.iter()
                    .map(|dp| dp.date.clone())
                    .collect();
                Ok(dates)
            }
            None => Ok(HashSet::new()),
        }
    }
}
