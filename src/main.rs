use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

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