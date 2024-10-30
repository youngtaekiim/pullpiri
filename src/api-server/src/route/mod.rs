// SPDX-License-Identifier: Apache-2.0

pub mod metric;
pub mod package;
pub mod scenario;

use axum::{body::Body, http::StatusCode, response::Response};

pub fn status_ok() -> Response<Body> {
    Response::builder()
        .status(StatusCode::OK)
        .body(Body::from("Ok".to_string()))
        .unwrap()
}

pub fn status_err() -> Response<Body> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from("Error".to_string()))
        .unwrap()
}
