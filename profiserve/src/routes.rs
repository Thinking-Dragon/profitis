use axum::{routing::get, Router};
use crate::handlers::{stocks, history};
use crate::state::AppState;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route(
            "/api/v1/stocks",
            get(stocks::get_stocks)
            .post(stocks::create_stock)
        )
        .route(
            "/api/v1/stocks/:ticker",
            get(stocks::get_stock)
            .put(stocks::update_stock)
            .delete(stocks::delete_stock)
        )
        .route(
            "/api/v1/stocks/:ticker/history",
            get(history::get_historical_data)
            .post(history::create_historical_data)
        )
        .route(
            "/api/v1/stocks/:ticker/history/:date",
            get(history::get_historical_data_point)
            .put(history::update_historical_data_point)
            .delete(history::delete_historical_data_point)
        )
        .with_state(state)
}
