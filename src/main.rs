use axum::{
    routing::get,
    Router
};
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub mod handler;
pub mod schema;
pub mod state;

#[tokio::main]
async fn main() {
    let atmos_entries = Router::new().route("/prueba", get( || async {"hola 1"}) );

    let api_v01 = Router::new().nest("/v01", atmos_entries);

    let app = Router::new().merge(api_v01).fallback(get(|| async {"ruta no valida"}));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

}

