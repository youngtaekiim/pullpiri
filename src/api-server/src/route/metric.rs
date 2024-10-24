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

pub async fn list_image() -> Json<NewImageList> {
    /*let s = common::etcd::get("metric/image").await.unwrap_or_default();
    let image: NewImageList = serde_json::from_str(&s).unwrap_or_default();*/
    let mut list: Vec<String> = Vec::new();
    let (_, v) = common::etcd::get_all("metric/image")
        .await
        .unwrap_or_default();
    for s in v {
        let mut each_list: NewImageList = serde_json::from_str(&s).unwrap_or_default();
        list.append(&mut each_list.images);
    }

    Json(NewImageList { images: list })
}

pub async fn list_container() -> Json<NewContainerList> {
    /*let s = common::etcd::get("metric/container")
        .await
        .unwrap_or_default();
    let container: NewContainerList = serde_json::from_str(&s).unwrap_or_default();*/
    let mut list: Vec<NewContainerInfo> = Vec::new();
    let (_, v) = common::etcd::get_all("metric/container")
        .await
        .unwrap_or_default();
    for s in v {
        let mut each_list: NewContainerList = serde_json::from_str(&s).unwrap_or_default();
        list.append(&mut each_list.containers);
    }
    Json(NewContainerList { containers: list })
}

pub async fn list_pod() -> Json<NewPodList> {
    /*let s = common::etcd::get("metric/pod").await.unwrap_or_default();
    let pod: NewPodList = serde_json::from_str(&s).unwrap_or_default();*/
    let mut list: Vec<NewPodInfo> = Vec::new();
    let (_, v) = common::etcd::get_all("metric/pod")
        .await
        .unwrap_or_default();
    for s in v {
        let mut each_list: NewPodList = serde_json::from_str(&s).unwrap_or_default();
        list.append(&mut each_list.pods);
    }
    Json(NewPodList { pods: list })
}

pub async fn list_scenario() -> Json<Vec<ScenarioInfo>> {
    let (mut k, mut v) = common::etcd::get_all("scenario").await.unwrap_or_default();

    let mut scenarios: Vec<ScenarioInfo> = Vec::new();
    let mut exist: Vec<String> = Vec::new();
    for _ in 0..k.len() {
        let key = k.pop().unwrap();
        let split: Vec<&str> = key.split('/').collect();

        let name = split.get(1).unwrap().to_string();
        if exist.contains(&name) {
            v.pop();
            continue;
        }
        exist.push(name.clone());
        let status = v.pop().unwrap_or_default();
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
pub struct ScenarioInfo {
    pub name: String,
    pub status: String,
    pub condition: String,
    pub action: String,
}
