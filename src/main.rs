use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
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
async fn main(){

}

async fn translate_handler(
    Json(payload): Json<TranslateRequest>,
) -> Result<Json<TranslateResponse>, StatusCode> {}