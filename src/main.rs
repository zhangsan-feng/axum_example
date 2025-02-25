
mod config;
mod middle;
mod entity;
mod api;
mod router;
mod mq;
mod interfaces;

use log::{info};

#[tokio::main]
async fn main() {
    config::config::config_init().await;

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000").await.unwrap();
    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, router::router::router_init().await).await.unwrap();
}




