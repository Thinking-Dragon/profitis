use std::collections::HashMap;
use std::sync::MutexGuard;

use axum::extract::State;
use axum::{
    extract::Path,
    http::StatusCode,
    Json,
};
use crate::models::Stock;
use crate::state::AppState;

pub async fn get_stocks(State(state): State<AppState>) -> Json<Vec<Stock>> {
    let stocks: MutexGuard<HashMap<String, Stock>> = state.stocks.lock().unwrap();
    let stock_list: Vec<Stock> = stocks.values().cloned().collect();
    Json(stock_list)
}

pub async fn create_stock(State(state): State<AppState>, Json(stock): Json<Stock>) -> (StatusCode, Json<Stock>) {
    let mut stocks: MutexGuard<HashMap<String, Stock>> = state.stocks.lock().unwrap();
    let ticker: String = stock.ticker.to_uppercase();
    let stock: Stock = Stock { 
        ticker: ticker.clone(),
        stock_exchange: stock.stock_exchange,
    };
    stocks.insert(ticker, stock.clone());
    (StatusCode::CREATED, Json(stock))
}

pub async fn get_stock(
    State(state): State<AppState>,
    Path(ticker): Path<String>,
) -> Result<Json<Stock>, StatusCode> {
    let stocks: MutexGuard<HashMap<String, Stock>> = state.stocks.lock().unwrap();
    let ticker: String = ticker.to_uppercase();

    stocks.get(&ticker)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

pub async fn update_stock(
    State(state): State<AppState>,
    Path(ticker): Path<String>,
    Json(updated_stock): Json<Stock>,
) -> Result<Json<Stock>, StatusCode> {
    let mut stocks: MutexGuard<HashMap<String, Stock>> = state.stocks.lock().unwrap();
    let ticker: String = ticker.to_uppercase();

    if stocks.contains_key(&ticker) {
        let stock: Stock = Stock { 
            ticker: updated_stock.ticker.to_uppercase(),
            stock_exchange: updated_stock.stock_exchange,
        };
        stocks.insert(ticker, stock.clone());
        Ok(Json(stock))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn delete_stock(
    State(state): State<AppState>,
    Path(ticker): Path<String>,
) -> StatusCode {
    let mut stocks: MutexGuard<HashMap<String, Stock>> = state.stocks.lock().unwrap();
    let ticker: String = ticker.to_uppercase();

    if stocks.remove(&ticker).is_some() {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}
