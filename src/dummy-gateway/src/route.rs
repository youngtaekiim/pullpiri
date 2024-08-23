use crate::scenario::ResourceScenario;
use axum::extract::State;
use axum_yaml::Yaml;
use tokio::sync::mpsc::Sender;

#[derive(serde::Serialize)]
pub struct Scenario {
    name: String,
}

pub async fn import_scenario(
    State(tx): State<Sender<ResourceScenario>>,
    Yaml(resource_scenario): Yaml<ResourceScenario>,
) -> String {
    let mut data: ResourceScenario = resource_scenario;
    let name = data.name.clone();
    data.route = Some(true);

    let _ = tx.send(data).await;
    format!("{name} is applied")
}

pub async fn delete_scenario(
    State(tx): State<Sender<ResourceScenario>>,
    Yaml(resource_scenario): Yaml<ResourceScenario>,
) -> String {
    let mut data: ResourceScenario = resource_scenario;
    let name = data.name.clone();
    data.route = Some(false);

    let _ = tx.send(data).await;
    format!("{name} is deleted")
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
