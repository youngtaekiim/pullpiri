// SPDX-License-Identifier: Apache-2.0

use axum::{
    extract::Path,
    response::IntoResponse,
    routing::{delete, get, post},
    Json, Router,
};

pub fn get_route() -> Router {
    Router::new()
        .route("/scenario", get(list_scenario))
        .route("/scenario/:scenario_name/:file_name", get(inspect_scenario))
        .route("/scenario", post(import_scenario))
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
        super::status_ok()
    } else {
        super::status_err()
    }
}

async fn import_scenario(body: String) -> impl IntoResponse {
    println!("POST : scenario {body} is called.\n");
    let scenario = importer::parse_scenario(&body).await;

    let scenario_path: Vec<&str> = body.split('/').collect();
    if write_scenario_info_to_etcd(scenario.unwrap(), scenario_path[1])
        .await
        .is_ok()
    {
        super::status_ok()
    } else {
        println!("error: writing scenario in etcd");
        super::status_err()
    }
}

async fn delete_scenario(
    Path((scenario_name, file_name)): Path<(String, String)>,
) -> impl IntoResponse {
    // TODO
    let path = format!("{scenario_name}/{file_name}");
    println!("todo - delete {path}");
    super::status_ok()
}

async fn write_scenario_info_to_etcd(
    s: importer::parser::scenario::Scenario,
    file_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let key_origin = format!("scenario/{}", s.name);
    common::etcd::put(&format!("{key_origin}/file"), file_name).await?;
    common::etcd::put(&format!("{key_origin}/actions"), &s.actions).await?;
    common::etcd::put(&format!("{key_origin}/conditions"), &s.conditions).await?;
    common::etcd::put(&format!("{key_origin}/targets"), &s.targets).await?;
    common::etcd::put(&format!("{key_origin}/full"), &s.scene).await?;

    let condition = common::gateway::Condition {
        name: format!("scenario/{}", &s.name),
    };

    crate::grpc::sender::gateway::send(condition).await?;

    Ok(())
}
