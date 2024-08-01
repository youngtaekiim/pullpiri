use axum::{
    extract::State,
    Json,
};
use tokio::sync::mpsc::UnboundedSender;

use crate::scenario::ResourceScenario;

#[derive(serde::Serialize)]
pub struct Scenario {
    name: String,
}

pub async fn import_scenario(
    State(tx): State<UnboundedSender<ResourceScenario>
) -> String {
    println!("POST : scenario {name} is called.\n");
    println!("       Path is {body}.\n");

}

/*
fn return_ok() -> Response<Body> {
    Response::builder()
        .status(StatusCode::OK)
        .body(Body::from("Ok".to_string()))
        .unwrap()
}

fn return_err() -> Response<Body> {
    Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body(Body::from("Error".to_string()))
        .unwrap()
}
*/
