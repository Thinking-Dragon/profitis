mod models;
mod state;
mod handlers;
mod routes;

use std::net::SocketAddr;
use state::AppState;
use routes::create_router;

#[tokio::main]
async fn main() {
    let state: AppState = AppState::new();
    let app: axum::Router = create_router(state);

    let addr: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://{}", addr);
    
    let listener: tokio::net::TcpListener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
