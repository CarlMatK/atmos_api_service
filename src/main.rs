pub mod dbmon;
pub mod handlers;
pub mod logger;
pub mod state;
#[cfg(test)]
pub mod tests;
pub mod types;

use axum::{Router, routing::post};
use handlers::handle_not_found;
use state::AppState;

use crate::handlers::{create_reading, list_readings};

#[tokio::main]
async fn main() {
    logger::implement();

    tracing::info!("Inicializando servicio atmos_api_service");
    let app = app().await;

    tracing::info!("Iniciando app en puerto 3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Fallo al iniciar en el puerto 3000");
    axum::serve(listener, app)
        .await
        .expect("No se pudo vincular el listener a la app");
}

async fn app() -> Router {
    tracing::info!("Construyendo router y endpoints");
    let reading_routes = Router::new().route("/data", post(create_reading).get(list_readings));
    let api_v01_routes = Router::new().nest("/v01", reading_routes);

    let app = Router::new()
        .merge(api_v01_routes)
        .fallback(handle_not_found)
        .with_state(AppState::new().await);

    tracing::debug!("Endpoints registrados: POST /v01/data, GET /v01/data");
    tracing::info!("Router listo");

    app
}
