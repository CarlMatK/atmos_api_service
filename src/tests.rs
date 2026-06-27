#[warn(unused_imports)]
use super::*;
use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};
use http_body_util::BodyExt;
use tower::ServiceExt;

#[tokio::test]
async fn trigger_fallback() {
    let app = app();
    let response = app
        .await
        .oneshot(Request::get("/wsd").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(&body[..], b"ruta no valida");
}

#[tokio::test]
async fn download_data() {
    let app = app();
    let response = app
        .await
        .oneshot(Request::get("/v01/data").body(Body::empty()).unwrap())
        .await
        .unwrap();

    let body: axum::body::Bytes = response.into_body().collect().await.unwrap().to_bytes();
    let s = String::from_utf8_lossy(&body).to_string();
    println!("{}", s);
}

#[tokio::test]
async fn upload_data() {
    let app = app();
    let info = types::AtmosReading::new();
    let gen_json = serde_json::to_string(&info).unwrap();
    let response = app
        .await
        .oneshot(
            Request::post("/v01/data")
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(gen_json.clone()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::ACCEPTED);
}
