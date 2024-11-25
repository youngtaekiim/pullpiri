// SPDX-License-Identifier: Apache-2.0

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
        let condition = common::etcd::get(&format!("scenario/{name}/conditions"))
            .await
            .unwrap();
        let action = common::etcd::get(&format!("scenario/{name}/actions"))
            .await
            .unwrap();
        scenarios.push(ScenarioInfo {
            name,
            status,
            condition,
            action,
        });
    }
    Json(scenarios)
}

#[derive(serde::Deserialize, serde::Serialize, Default)]
struct ScenarioInfo {
    name: String,
    status: String,
    condition: String,
    action: String,
}
