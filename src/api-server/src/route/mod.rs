// SPDX-License-Identifier: Apache-2.0

pub mod metric;
pub mod package;
pub mod scenario;

use axum::{http::StatusCode, Json};

#[derive(serde::Serialize)]
pub struct ResponseData {
    resp: String,
}

pub fn status_ok() -> (StatusCode, Json<ResponseData>) {
    let response = ResponseData {
        resp: "Ok".to_string(),
    };
    (StatusCode::OK, Json(response))
}

pub fn status_err(msg: &str) -> (StatusCode, Json<ResponseData>) {
    let response = ResponseData {
        resp: msg.to_string(),
    };
    (StatusCode::NOT_FOUND, Json(response))
}

/*pub fn status_ok() -> Response<Body> {
    Response::builder()
        .status(StatusCode::OK)
        .body(Body::from("Ok".to_string()))
        .unwrap()
}

pub fn status_err(msg: &str) -> Response<Body> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from(msg.to_string()))
        .unwrap()
}*/
