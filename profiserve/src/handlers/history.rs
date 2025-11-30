use std::collections::HashMap;
use std::sync::MutexGuard;

use axum::extract::State;
use axum::{
    extract::Path,
    http::StatusCode,
    Json,
};
use crate::models::{HistoricalDataPoint, HistoricalDataList};
use crate::state::AppState;

pub async fn get_historical_data(
    State(state): State<AppState>,
    Path(ticker): Path<String>,
) -> Result<Json<HistoricalDataList>, StatusCode> {
    let historical_data: MutexGuard<HashMap<String, Vec<HistoricalDataPoint>>> = state.historical_data.lock().unwrap();
    let ticker: String = ticker.to_uppercase();

    historical_data.get(&ticker)
        .cloned()
        .map(|data: Vec<HistoricalDataPoint>| Json(HistoricalDataList { ticker: ticker.clone(), data }))
        .ok_or(StatusCode::NOT_FOUND)
}

pub async fn create_historical_data(
    State(state): State<AppState>,
    Path(ticker): Path<String>,
    Json(data_point): Json<HistoricalDataPoint>,
) -> (StatusCode, Json<HistoricalDataPoint>) {
    let mut historical_data: MutexGuard<HashMap<String, Vec<HistoricalDataPoint>>> = state.historical_data.lock().unwrap();
    let ticker: String = ticker.to_uppercase();

    historical_data.entry(ticker)
        .or_insert_with(Vec::<HistoricalDataPoint>::new)
        .push(data_point.clone());
    (StatusCode::CREATED, Json(data_point))
}

pub async fn get_historical_data_point(
    State(state): State<AppState>,
    Path((ticker, date)): Path<(String, String)>,
) -> Result<Json<HistoricalDataPoint>, StatusCode> {
    let historical_data: MutexGuard<HashMap<String, Vec<HistoricalDataPoint>>> = state.historical_data.lock().unwrap();
    let ticker: String = ticker.to_uppercase();

    if let Some(data_points) = historical_data.get(&ticker) {
        data_points.iter()
            .find(|dp: &&HistoricalDataPoint| dp.date == date)
            .cloned()
            .map(Json)
            .ok_or(StatusCode::NOT_FOUND)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn update_historical_data_point(
    State(state): State<AppState>,
    Path((ticker, date)): Path<(String, String)>,
    Json(updated_data): Json<HistoricalDataPoint>,
) -> Result<Json<HistoricalDataPoint>, StatusCode> {
    let mut historical_data: MutexGuard<HashMap<String, Vec<HistoricalDataPoint>>> = state.historical_data.lock().unwrap();
    let ticker: String = ticker.to_uppercase();

    if let Some(data_points) = historical_data.get_mut(&ticker) {
        if let Some(point) = data_points.iter_mut().find(|dp: &&mut HistoricalDataPoint| dp.date == date) {
            *point = updated_data.clone();
            Ok(Json(updated_data))
        } else {
            Err(StatusCode::NOT_FOUND)
        }
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn delete_historical_data_point(
    State(state): State<AppState>,
    Path((ticker, date)): Path<(String, String)>,
) -> StatusCode {
    let mut historical_data: MutexGuard<HashMap<String, Vec<HistoricalDataPoint>>> = state.historical_data.lock().unwrap();
    let ticker: String = ticker.to_uppercase();
    if let Some(data_points) = historical_data.get_mut(&ticker) {
        if let Some(pos) = data_points.iter().position(|dp: &HistoricalDataPoint| dp.date == date) {
            data_points.remove(pos);
            StatusCode::NO_CONTENT
        } else {
            StatusCode::NOT_FOUND
        }
    } else {
        StatusCode::NOT_FOUND
    }
}
