// SPDX-License-Identifier: Apache-2.0

use axum::{
    body::Body,
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{delete, get, post},
    Json, Router,
};

pub fn get_route() -> Router {
    Router::new()
        .route("/scenario/", get(list_scenario))
        .route("/scenario/:scenario_name/:file_name", get(inspect_scenario))
        .route("/scenario/:scenario_name/:file_name", post(import_scenario))
        .route(
            "/scenario/:scenario_name/:file_name",
            delete(delete_scenario),
        )
}

async fn list_scenario() -> Json<Vec<String>> {
    // TODO
    let scenarios = vec![String::new(), String::new()];
    Json(scenarios)
}

async fn inspect_scenario(
    Path((scenario_name, file_name)): Path<(String, String)>,
) -> impl IntoResponse {
    let key = format!("scenario/{scenario_name}/file");
    let v = common::etcd::get(&key).await.unwrap_or_default();

    if file_name == v {
        return_ok()
    } else {
        return_err()
    }
}

async fn import_scenario(
    Path((scenario_name, file_name)): Path<(String, String)>,
    body: String,
) -> impl IntoResponse {
    let scenario = importer::parse_scenario(&body).await;

    println!("POST : scenario {scenario_name} is called.\n");
    println!("       Path is {body}.\n");

    if write_scenario_info_to_etcd(scenario.unwrap(), &file_name)
        .await
        .is_ok()
    {
        return_ok()
    } else {
        println!("error: writing scenario in etcd");
        return_err()
    }
}

async fn delete_scenario(
    Path((scenario_name, file_name)): Path<(String, String)>,
) -> impl IntoResponse {
    // TODO
    let path = format!("{scenario_name}/{file_name}");
    println!("todo - delete {path}");
    return_ok()
}

fn return_ok() -> Response<Body> {
    Response::builder()
        .status(StatusCode::OK)
        .body(Body::from("Ok".to_string()))
        .unwrap()
}

fn return_err() -> Response<Body> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from("Error".to_string()))
        .unwrap()
}

use common::gateway;
use importer::parser::scenario::Scenario;

async fn write_scenario_info_to_etcd(
    s: Scenario,
    file_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let key_origin = format!("scenario/{}", s.name);
    common::etcd::put(&format!("{key_origin}/file"), file_name).await?;
    common::etcd::put(&format!("{key_origin}/actions"), &s.actions).await?;
    common::etcd::put(&format!("{key_origin}/conditions"), &s.conditions).await?;
    common::etcd::put(&format!("{key_origin}/targets"), &s.targets).await?;
    common::etcd::put(&format!("{key_origin}/full"), &s.scene).await?;

    let condition = gateway::Condition {
        name: format!("scenario/{}", &s.name),
    };

    crate::grpc::sender::gateway::send(condition).await?;

    Ok(())
}
