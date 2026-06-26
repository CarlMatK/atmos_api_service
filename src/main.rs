#[cfg(test)]
pub mod tests;
pub mod dbmon;
pub mod handlers;
pub mod state;
pub mod types;

use axum::{Router, routing::post};
use handlers::fallbacker;
use state::AppState;

use crate::handlers::{download, upload_data};

#[tokio::main]
async fn main() {
    let app = app().await;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("No se inicializar al puerto 3000");
    axum::serve(listener, app)
        .await
        .expect("No se pudo vincular el listener a la app");
}

async fn app() -> Router {
    let atmos_endpoints = Router::new().route("/data", post(upload_data).get(download));
    let v01_endpoints = Router::new().nest("/v01", atmos_endpoints);

    Router::new()
        .merge(v01_endpoints)
        .fallback(fallbacker)
        .with_state(AppState::new().await)
}
