mod error;
mod extractors;
mod handlers;
mod openapi;
mod serialization;

use axum::{
    routing::{get, post},
    Router,
};
use dotenv::dotenv;
use extractors::state::AppState;
use handlers::*;
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() {
    dotenv().expect("fail to load env");

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_url = std::env::var("DATABASE_URL").expect("missing DATABASE_URL");

    let app_state = AppState::new(db_url).await.expect("fail to init app state");

    let app = Router::new()
        .merge(
            SwaggerUi::new("/api/docs").url("/api-docs/openapi.json", openapi::ApiDoc::openapi()),
        )
        .route("/api", get(|| async { "hello 🦀!" }))
        .route("/api/events", get(get_events))
        .route("/api/event/:id", get(get_event))
        .route("/api/sign-in", post(sign_in))
        .layer(CorsLayer::permissive())
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    tracing::info!(
        "🦀 server is listening on {}",
        listener.local_addr().unwrap()
    );

    axum::serve(listener, app).await.unwrap();
}
