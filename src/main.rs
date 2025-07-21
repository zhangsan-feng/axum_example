
mod config;
mod middle;
mod entity;
mod api;
mod router;
mod mq;
mod interfaces;
mod utils;
mod external;
mod internal;
use std::env;
use log::{info};

#[tokio::main]
async fn main() {
    config::logger::logger_init(env::current_dir().unwrap().display().to_string()).await;

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000").await.unwrap();
    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, router::router::router_init().await).await.unwrap();
}



/*
   https://crates.io/

*/
