use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use trad::Translator;

#[derive(Deserialize, Debug)]
struct TranslateRequest {
    sauce: String,
    target: String,
    text: String,
}

#[derive(Serialize, Debug)]
struct TranslateResponse {
    text: String,
}

#[tokio::main]
async fn main()-> Result <() Box<dyn std::error:Error>>{
    let translator = Translator::setup(None).await?;

    let port = 5345;

    let app = Router::new()
        .route("/translate", post(translate_handler))
        .route("/status", get(status_handler));
    
    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

async fn translate_handler(
    Json(payload): Json<TranslateRequest>,
) -> Result<Json<TranslateResponse>, StatusCode> {
    let translator = Translator::default();

    match translator.translate(&payload.sauce, &payload.target, &payload.text).await {
        Ok(translated_text) => Ok(Json(TranslateResponse {
            text: translated_text,
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn status_handler() -> StatusCode {
    let translator = Translator::default();
    let start_time = std::time::Instant::now();
    let result = translator.translate("en", "ja", "healthcheck").await;
    let duration = start_time.elapsed();



    match result {
        Ok(_) => {
            if duration.as_secs() > 3 {
                StatusCode::SERVICE_UNAVAILABLE
            } else {
                StatusCode::OK
            }
        }
        Err(_) => StatusCode::SERVICE_UNAVAILABLE,
    }
}