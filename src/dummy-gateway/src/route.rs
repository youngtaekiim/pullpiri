use crate::scenario::ResourceScenario;
use axum::{extract::State, Json};
use tokio::sync::mpsc::Sender;

#[derive(serde::Serialize)]
pub struct Scenario {
    name: String,
}

pub async fn import_scenario(
    State(tx): State<Sender<ResourceScenario>>,
    Json(resource_scenario): Json<ResourceScenario>,
) -> String {
    let data: ResourceScenario = resource_scenario;
    let name = data.name.clone();

    let _ = tx.send(data).await;
    name
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
