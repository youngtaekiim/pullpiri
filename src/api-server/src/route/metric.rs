// SPDX-License-Identifier: Apache-2.0

use crate::grpc::receiver::metric_notifier::{
    NewContainerInfo, NewContainerList, NewImageList, NewPodInfo, NewPodList,
};
use axum::{routing::get, Json, Router};
use std::collections::HashMap;

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
    use std::collections::HashSet;

    let kvs = common::etcd::get_all_with_prefix("scenario")
        .await
        .unwrap_or_default();
    let mut scenarios: Vec<ScenarioInfo> = Vec::new();

    let mut exist: HashSet<&str> = HashSet::new();
    for kv in kvs.iter() {
        let split: Vec<&str> = kv.key.split('/').collect();
        let name = match split.get(1) {
            Some(&item) => item,
            None => continue,
        };
        if !exist.insert(name) {
            continue;
        }

        let status = common::etcd::get(&format!("scenario/{name}/status"))
            .await
            .unwrap();

        let mut metric_condition = HashMap::new();
        let condition_str = common::etcd::get(&format!("scenario/{name}/conditions"))
            .await
            .unwrap();
        if let Ok(condition) =
            serde_yaml::from_str::<common::spec::scenario::Condition>(&condition_str)
        {
            metric_condition.insert(
                capitalize_first_letter(&condition.get_operand_name()),
                capitalize_first_letter_only(&condition.get_value()),
            );
        }

        // TODO - temp code
        let mut action = HashMap::new();
        if name.contains("high-performance") {
            action.insert("Power".to_string(), "On".to_string());
        } else if name.contains("eco") {
            action.insert("Eco".to_string(), "On".to_string());
        } else if name.contains("enable") {
            action.insert("antipinch-enable".to_string(), "v2.0".to_string());
        } else if name.contains("disable") {
            action.insert("antipinch-disable".to_string(), "v1.0".to_string());
        }

        scenarios.push(ScenarioInfo {
            name: String::from(name),
            status,
            condition: metric_condition,
            action,
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
    condition: HashMap<String, String>,
    action: HashMap<String, String>,
}
