use axum::http::{HeaderValue, Method};
use axum::{middleware, Router};
use axum::routing::get;
use tower_http::cors::CorsLayer;
use crate::api::user::api_params_example::get_params_example;
use crate::middle::request_record::request_record;

#[derive(Debug, Clone)]
struct AppState {
    // pub mysql_client:Arc<MySqlPool>,
    // pub redis_client:Arc<RedisClient>,
    // pub pgsql_client:Arc<PgPool>
}



pub async fn router_init() -> Router<()> {


    let app_state = AppState{


    };
    let app = Router::new()
    .nest("/api",
            Router::new().nest("/user",
                Router::new()
                    .route("/get_params_example", get(get_params_example))
                    // .route("/user_edit", get(user_edit))
                    // .route("/user_info", get(user_info))

            )
        )
        .layer(middleware::from_fn(request_record))
        .layer(
            CorsLayer::new()
                .allow_origin("*".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST])
        )
        .with_state(app_state);

    app
}