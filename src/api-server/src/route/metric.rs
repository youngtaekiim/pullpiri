// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;

use axum::{routing::get, Json, Router};

use crate::grpc::receiver::metric_notifier::{
    NewContainerInfo, NewContainerList, NewImageList, NewPodInfo, NewPodList,
};

pub fn get_route() -> Router {
    Router::new()
        .route("/metric/image", get(list_image))
        .route("/metric/container", get(list_container))
        .route("/metric/pod", get(list_pod))
        .route("/metric/scenario", get(list_scenario))
}

async fn list_image() -> Json<NewImageList> {
    let mut list: Vec<String> = Vec::new();
    let kvs = common::etcd::get_all_with_prefix("metric/image")
        .await
        .unwrap_or_default();
    for kv in kvs {
        let mut each_list: NewImageList = serde_json::from_str(&kv.value).unwrap_or_default();
        list.append(&mut each_list.images);
    }

    Json(NewImageList { images: list })
}

async fn list_container() -> Json<NewContainerList> {
    let mut list: Vec<NewContainerInfo> = Vec::new();
    let kvs = common::etcd::get_all_with_prefix("metric/container")
        .await
        .unwrap_or_default();
    for kv in kvs {
        let mut each_list: NewContainerList = serde_json::from_str(&kv.value).unwrap_or_default();
        list.append(&mut each_list.containers);
    }
    Json(NewContainerList { containers: list })
}

async fn list_pod() -> Json<NewPodList> {
    let mut list: Vec<NewPodInfo> = Vec::new();
    let kvs = common::etcd::get_all_with_prefix("metric/pod")
        .await
        .unwrap_or_default();
    for kv in kvs {
        let mut each_list: NewPodList = serde_json::from_str(&kv.value).unwrap_or_default();
        list.append(&mut each_list.pods);
    }
    Json(NewPodList { pods: list })
}

async fn list_scenario() -> Json<Vec<ScenarioInfo>> {
    let mut kvs = common::etcd::get_all_with_prefix("scenario")
        .await
        .unwrap_or_default();
    let mut scenarios: Vec<ScenarioInfo> = Vec::new();
    let mut exist: Vec<String> = Vec::new();

    for _ in 0..kvs.len() {
        let kv = kvs.pop().unwrap();
        let split: Vec<&str> = kv.key.split('/').collect();

        let name = split.get(1).unwrap().to_string();
        if exist.contains(&name) {
            continue;
        }

        exist.push(name.clone());
        let status = common::etcd::get(&format!("scenario/{name}/status"))
            .await
            .unwrap();
        /*let condition = common::etcd::get(&format!("scenario/{name}/conditions"))
            .await
            .unwrap();
        let action = common::etcd::get(&format!("scenario/{name}/actions"))
            .await
            .unwrap();*/

        // TODO - temporary code

        //let mut metric_condition = MetricCondition::default();
        let mut metric_condition = HashMap::new();
        let conditions = common::etcd::get(&format!("scenario/{name}/conditions"))
            .await
            .unwrap();
        if let Ok(condition) =
            serde_yaml::from_str::<common::spec::scenario::Condition>(&conditions)
        {
            /*metric_condition = MetricCondition {
                name: capitalize_first_letter(&condition.get_operand_name()),
                value: capitalize_first_letter_only(&condition.get_value()),
            }*/
            metric_condition.insert(
                capitalize_first_letter(&condition.get_operand_name()),
                capitalize_first_letter_only(&condition.get_value()),
            );
        }

        //let mut metric_action = MetricAction::default();
        let mut metric_action = HashMap::new();
        if name.contains("high-performance") {
            /*metric_action = MetricAction {
                name: "Power".to_string(),
                value: "On".to_string(),
            }*/
            metric_action.insert("Power".to_string(), "On".to_string());
        } else if name.contains("eco") {
            /*metric_action = MetricAction {
                name: "Eco".to_string(),
                value: "On".to_string(),
            }*/
            metric_action.insert("Eco".to_string(), "On".to_string());
        } else if name.contains("enable") {
            /*metric_action = MetricAction {
                name: "antipinch-enable".to_string(),
                value: "v2.0".to_string(),
            }*/
            metric_action.insert("antipinch-enable".to_string(), "v2.0".to_string());
        } else if name.contains("disable") {
            /*metric_action = MetricAction {
                name: "antipinch-disable".to_string(),
                value: "v1.0".to_string(),
            }*/
            metric_action.insert("antipinch-disable".to_string(), "v1.0".to_string());
        }

        scenarios.push(ScenarioInfo {
            name,
            status,
            condition: metric_condition,
            action: metric_action,
        });
    }
    Json(scenarios)
}

fn capitalize_first_letter(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

fn capitalize_first_letter_only(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>(),
        None => String::new(),
    }
}

#[derive(serde::Deserialize, serde::Serialize, Default)]
struct ScenarioInfo {
    name: String,
    status: String,
    //condition: String,
    //conditions: common::spec::scenario::Condition,
    //condition: MetricCondition,
    condition: HashMap<String, String>,
    //action: String,
    //action: MetricAction,
    action: HashMap<String, String>,
}

#[derive(serde::Deserialize, serde::Serialize, Default)]
struct MetricCondition {
    name: String,
    value: String,
}

#[derive(serde::Deserialize, serde::Serialize, Default)]
struct MetricAction {
    name: String,
    value: String,
}
