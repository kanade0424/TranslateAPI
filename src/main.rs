use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
    extract::State,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use trad::{Translator,Language,languages};
use std::sync::Arc;
#[derive(Deserialize, Debug)]
struct TranslateRequest {
    source: String,
    target: String,
    text: String,
}

#[derive(Serialize, Debug)]
struct TranslateResponse {
    text: String,
}

#[tokio::main]
async fn main()-> Result <(), Box<dyn std::error::Error>>{
    let translator = Arc::new(Translator::setup(None).await?);

    let port = 5345;

    let app = Router::new()
        .route("/translate", post(translate_handler))
        .route("/status", get(status_handler))
        .with_state(translator);
    
    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("ポート{}でAPIサーバーが起動しました。",port);
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

async fn translate_handler(
    State(translator): State<Arc<Translator>>,
    Json(payload): Json<TranslateRequest>,
) -> Result<Json<TranslateResponse>, StatusCode> {
    let source_lang = lang_maping(&payload.source)?;
    let target_lang = lang_maping(&payload.target)?;

    match translator.translate(&payload.text, source_lang, target_lang).await {
        Ok(translated_text) => Ok(Json(TranslateResponse {
            text: translated_text,
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn status_handler(State(translator): State<Arc<Translator>>,) -> StatusCode {
    let start_time = std::time::Instant::now();
    let result = translator.translate("healthcheck", "en", "ja").await;
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

fn lang_maping(code: &str) -> Result<trad::Language, StatusCode> {
    match code.to_lowercase().as_str() {
        "ja" => Ok(languages::JAPANESE),
        "en" => Ok(languages::ENGLISH),
        "de" => Ok(languages::GERMAN),
        "ko" => Ok(languages::KOREAN),
        "zh" => Ok(languages::CHINESE),
        "fr" => Ok(languages::FRENCH),
        "pt" => Ok(languages::PORTUGUESE),
        _ => Err(StatusCode::BAD_REQUEST), // 未対応の言語コードが来たら400を返す
    }
}