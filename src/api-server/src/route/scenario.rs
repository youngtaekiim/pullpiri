// SPDX-License-Identifier: Apache-2.0

use axum::{
    body::Body,
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{delete, get, post},
    Json, Router,
};

#[derive(serde::Serialize)]
pub struct Scenario {
    name: String,
}

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

pub async fn list_scenario() -> Json<Vec<Scenario>> {
    let scenarios = vec![
        Scenario {
            name: "version".to_string(),
        },
        Scenario {
            name: "display".to_string(),
        },
    ];
    Json(scenarios)
}

pub async fn inspect_scenario(
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

pub async fn import_scenario(
    Path((scenario_name, file_name)): Path<(String, String)>,
    body: String,
) -> impl IntoResponse {
    let scenario = importer::handle_scenario(&body).await;

    println!("POST : scenario {scenario_name} is called.\n");
    println!("       Path is {body}.\n");

    /*if scenario.is_err() {
        return return_err();
    }*/

    if crate::manager::handle_scenario_msg(scenario.unwrap(), &file_name)
        .await
        .is_err()
    {
        println!("error: writing scenario in etcd");
        return return_err();
    }

    return_ok()
}

pub async fn delete_scenario(
    Path((scenario_name, file_name)): Path<(String, String)>,
) -> impl IntoResponse {
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
