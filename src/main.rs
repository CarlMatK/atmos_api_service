use axum::{
    Router,
extract::ConnectInfo,
    routing::{get, post},
    Json,
};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub mod handler;
pub mod state;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

   
    let app = app();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.expect("No se inicializar al puerto 3000");
    axum::serve(listener, app).await.expect("No se pudo vincular el listener a la app");

}

fn app() -> Router {
    let atmos_entries = Router::new().route("/prueba", get( || async {"hola 1"}) );
    let api_v01 = Router::new().nest("/v01", atmos_entries);
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .merge(api_v01).fallback(get(|| async {"ruta no valida"}))
        // We can still add middleware
        //.layer(TraceLayer::new_for_http())
}


#[cfg(test)]
mod tests {
    use super::*;
     use axum::{
        body::Body,
        extract::connect_info::MockConnectInfo,
        http::{self, Request, StatusCode},
    };
    use http_body_util::BodyExt; // for `collect`
    use serde_json::{json, Value};
    use tokio::net::TcpListener;
    use tower::{Service, ServiceExt}; // for `call`, `oneshot`, and `ready`
    
    #[tokio::test]
    async fn get_hello_world() {
        let app = app();
        let response = app
            .oneshot(Request::get("/").body(Body::empty()).unwrap())
            .await
            .unwrap();
          assert_eq!(response.status(), StatusCode::OK);
          let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], b"Hello, World!");
    }
    
}