// SPDX-License-Identifier: Apache-2.0

use axum::{
    extract::Path,
    response::Response,
    routing::{delete, get, post},
    Json, Router,
};

pub fn get_route() -> Router {
    Router::new()
        .route("/scenario", get(list_scenario))
        .route("/scenario/:scenario_name/:file_name", get(inspect_scenario))
        .route("/scenario", post(handle_post_path))
        .route("/scenario/yaml", post(handle_post_yaml))
        .route("/scenario/:scenario_name", delete(handle_delete))
        // temporary
        .route("/scenario/reset", get(reset_all))
}

async fn list_scenario() -> Json<Vec<String>> {
    // TODO - /metric/scenario will be moved here
    let scenarios = vec![String::new(), String::new()];
    Json(scenarios)
}

async fn inspect_scenario(Path((scenario_name, file_name)): Path<(String, String)>) -> Response {
    let key = format!("scenario/{scenario_name}/file");
    let v = common::etcd::get(&key).await.unwrap_or_default();

    if file_name == v {
        super::status_ok()
    } else {
        super::status_err("file does not exist in etcd")
    }
}

async fn handle_post_path(body: String) -> Response {
    println!("\nPOST : scenario {body} is called.");
    let result = import_scenario_from_path(body).await;

    if let Err(msg) = result {
        super::status_err(&msg.to_string())
    } else {
        super::status_ok()
    }
}

async fn handle_post_yaml(body: String) -> Response {
    println!("\nPOST : maked scenario is called.");
    let result = import_scenario_from_yaml(body).await;

    if let Err(msg) = result {
        super::status_err(&msg.to_string())
    } else {
        super::status_ok()
    }
}

async fn import_scenario_from_path(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let scenario = importer::get_scenario_from_file(&path).await?;
    let scenario_file = path.split('/').collect::<Vec<&str>>()[1];
    internal_import_scenario(&scenario, scenario_file).await
}

async fn import_scenario_from_yaml(yaml: String) -> Result<(), Box<dyn std::error::Error>> {
    let scenario = importer::get_scenario_from_file(&yaml).await?;
    internal_import_scenario(&scenario, &scenario.name).await
}

async fn internal_import_scenario(
    s: &importer::parser::scenario::Scenario,
    file_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    write_scenario_info_in_etcd(s, file_name).await?;
    let condition = common::gateway::Condition {
        crud: String::from("CREATE"),
        name: file_name.to_string(),
    };
    crate::grpc::sender::gateway::send(condition).await?;

    Ok(())
}

async fn handle_delete(Path(file_name): Path<String>) -> Response {
    println!("DELETE : scenario {file_name} is called.\n");
    let result = delete_scenario(&file_name).await;

    if let Err(msg) = result {
        super::status_err(&msg.to_string())
    } else {
        super::status_ok()
    }
}

async fn delete_scenario(file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    delete_scenario_info_in_etcd(file_name).await?;

    let condition = common::gateway::Condition {
        crud: "DELETE".to_string(),
        name: file_name.to_string(),
    };
    crate::grpc::sender::gateway::send(condition).await?;

    Ok(())
}

async fn write_scenario_info_in_etcd(
    s: &importer::parser::scenario::Scenario,
    file_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    //let key_origin = format!("scenario/{}", s.name);
    let key_origin = format!("scenario/{}", file_name);
    //common::etcd::put(&format!("{key_origin}/file"), file_name).await?;
    common::etcd::put(&format!("{key_origin}/actions"), &s.actions).await?;
    common::etcd::put(&format!("{key_origin}/conditions"), &s.conditions).await?;
    common::etcd::put(&format!("{key_origin}/status"), "inactive").await?;
    common::etcd::put(&format!("{key_origin}/targets"), &s.targets).await?;
    common::etcd::put(&format!("{key_origin}/full"), &s.scene).await?;

    Ok(())
}

async fn delete_scenario_info_in_etcd(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let key_prefix = format!("scenario/{}", name);
    common::etcd::delete_all_with_prefix(&key_prefix).await?;

    Ok(())
}

// emergency reset button
async fn reset_all() -> Response {
    // TODO - run shell script?
    super::status_ok()
}
