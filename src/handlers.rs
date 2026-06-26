use std::sync::Arc;
use crate::types::AtmosData;
use crate::{AppState};
use axum::{Json, extract::State, http::StatusCode};

pub async fn fallbacker() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "ruta no valida")
}
//* todo extractor que quiera convertir de json a un struct necesito deserializarlo con serde_json

pub async fn upload_data(
    State(state): State<Arc<AppState>>,
    Json(data): Json<AtmosData>,
) -> StatusCode {
    state.rerun_stamp();
    let (str1, str2) = state.stamp_intoparts();
    state.upd_battery(data.into_batt());
    state.pool.push_from_param(data, str1, str2).await;
    StatusCode::ACCEPTED
}

pub async fn download(State(state): State<Arc<AppState>>) -> String {
    state.pool.query_all().await
}
