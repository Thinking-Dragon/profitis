use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Stock {
    pub ticker: String,
    pub stock_exchange: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct HistoricalDataPoint {
    pub date: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: u64,
}

#[derive(Serialize)]
pub struct HistoricalDataList {
    pub ticker: String,
    pub data: Vec<HistoricalDataPoint>,
}
