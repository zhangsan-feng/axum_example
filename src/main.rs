
mod config;
mod _middleware;
mod entity;
mod api;

use std::sync::Arc;
use std::time::Duration;
use axum::{BoxError, Json, middleware::{self}, Router, routing::post, ServiceExt};
use axum::error_handling::HandleErrorLayer;
use axum::handler::HandlerWithoutStateExt;
use axum::http::{HeaderValue, Method, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use log::info;
use tokio::sync::RwLock;
use tower::{MakeService, ServiceBuilder};

use tower_http::cors::CorsLayer;
use tower_http::timeout::TimeoutLayer;


#[tokio::main]
async fn main() {
    let app_state = Arc::new(RwLock::new(AppState::default()));


    let app = Router::new().nest("/v1",
        Router::new().nest("/web",
            Router::new().nest("/user",
                Router::new()

                    .route("/info", get(user_info))
                    .route("/login", post(user_login))
                    .route("/register", post(user_register))
                    .route("/register2", post(download_resource))
                // .route("/file", post(accept_form))

            )))
        // .layer(_middleware::from_fn(_middleware::request_record::request_record))
        .layer(
            CorsLayer::new()
                .allow_origin("*".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET]),
        ).with_state(Arc::clone(&app_state));



    // app.layer();
    // app.handle_error()
    // app.fallback()


    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}




