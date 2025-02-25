use axum::http::{HeaderValue, Method};
use axum::Router;
use axum::routing::post;
use tower_http::cors::CorsLayer;
use crate::api::user::login::user_login;

#[derive()]
struct AppState {}


pub async fn router_init() -> Router<()> {
    // let app_state = Arc::new(RwLock::new(AppState::default()));
    let app = Router::new()
    .nest("/v1",
          Router::new().nest("/web",
                             Router::new().nest("/user",
                                                Router::new().route("/login", post(user_login))
                             )
          )
    )

    .layer(
        CorsLayer::new().
        allow_origin("*".parse::<HeaderValue>().unwrap()).
        allow_methods([Method::GET, Method::POST])
    );


    // with_state(Arc::clone(&app_state));
    // app.layer();
    // app.handle_error()
    // app.fallback()
    app
}