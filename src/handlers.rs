use std::sync::Arc;

use crate::AppState;
use crate::types::AtmosReading;
use axum::{Json, extract::State, http::StatusCode};
use tracing::instrument;

pub async fn handle_not_found() -> (StatusCode, &'static str) {
    tracing::debug!("Se recibio una peticion hacia una ruta no registrada");
    (StatusCode::NOT_FOUND, "ruta no valida")
}
//* todo extractor que quiera convertir de json a un struct necesito deserializarlo con serde_json

#[instrument(skip(state, data), fields(battery_level = data.battery_level()))]
pub async fn create_reading(
    State(state): State<Arc<AppState>>,
    Json(data): Json<AtmosReading>,
) -> StatusCode {
    let battery_level = data.battery_level();
    tracing::info!("Procesando carga de lectura atmosferica");
    state.refresh_time();
    let (date, time) = state.timestamp_parts();
    tracing::debug!(
        battery_level,
        date = %date,
        time = %time,
        "Estado actualizado antes de persistir la lectura"
    );
    state.update_battery(battery_level);
    state.pool.insert_from_param(data, date, time).await;
    tracing::info!("Lectura almacenada correctamente");
    StatusCode::ACCEPTED
}

#[instrument(skip(state))]
pub async fn list_readings(State(state): State<Arc<AppState>>) -> String {
    tracing::info!("Procesando consulta de lecturas atmosfericas");
    state.pool.fetch_all_readings().await
}
