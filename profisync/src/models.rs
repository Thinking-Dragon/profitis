use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Stock {
    pub ticker: String,
    pub stock_exchange: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HistoricalDataPoint {
    pub date: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HistoricalDataList {
    pub ticker: String,
    pub data: Vec<HistoricalDataPoint>,
}

#[derive(Deserialize, Debug)]
pub struct YahooFinanceResponse {
    pub chart: Chart,
}

#[derive(Deserialize, Debug)]
pub struct Chart {
    pub result: Vec<ChartResult>,
}

#[derive(Deserialize, Debug)]
pub struct ChartResult {
    pub timestamp: Vec<i64>,
    pub indicators: Indicators,
}

#[derive(Deserialize, Debug)]
pub struct Indicators {
    pub quote: Vec<Quote>,
}

#[derive(Deserialize, Debug)]
pub struct Quote {
    pub open: Vec<Option<f64>>,
    pub high: Vec<Option<f64>>,
    pub low: Vec<Option<f64>>,
    pub close: Vec<Option<f64>>,
    pub volume: Vec<Option<u64>>,
}
