use std::sync::Arc;
use axum::http::{HeaderValue, Method};
use axum::Router;
use axum::routing::post;
use tower_http::cors::CorsLayer;
use redis::Client as RedisClient;
use sea_orm::sqlx::{MySqlPool, PgPool};
use crate::api::user::login::user_login;


#[derive(Debug, Clone)]
struct AppState {
    pub mysql_client:Arc<MySqlPool>,
    pub redis_client:Arc<RedisClient>,
    pub pgsql_client:Arc<PgPool>
}


pub async fn router_init() -> Router<()> {


    let app_state = AppState{
        mysql_client: Arc::new(MySqlPool::connect("").await.expect("")),
        redis_client: Arc::new(RedisClient::open("").expect("Failed to create Redis client")),
        pgsql_client: Arc::new(PgPool::connect("").await.expect("")),
    };
    let app = Router::new()
    .nest("/api",
            Router::new().nest("/user",
                               Router::new().route("/login", post(user_login))
                     )

    )
    .layer(
        CorsLayer::new().
        allow_origin("*".parse::<HeaderValue>().unwrap()).
        allow_methods([Method::GET, Method::POST])
    )
    .with_state(Arc::clone(&app_state));

    // app.layer();
    // app.handle_error()
    // app.fallback()
    app
}