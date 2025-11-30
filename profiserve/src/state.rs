use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::models::{Stock, HistoricalDataPoint};

pub type StockStore = Arc<Mutex<HashMap<String, Stock>>>;
pub type HistoricalDataStore = Arc<Mutex<HashMap<String, Vec<HistoricalDataPoint>>>>;

#[derive(Clone)]
pub struct AppState {
    pub stocks: StockStore,
    pub historical_data: HistoricalDataStore,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            stocks: Arc::new(Mutex::new(HashMap::<String, Stock>::new())),
            historical_data: Arc::new(Mutex::new(HashMap::<String, Vec<HistoricalDataPoint>>::new())),
        }
    }
}
