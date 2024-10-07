use axum::{routing::get, Json, Router};

use crate::grpc::receiver::metric_notifier::{NewContainerList, NewImageList, NewPodList};

pub fn get_route() -> Router {
    Router::new()
        .route("/metric/image", get(list_image))
        .route("/metric/container", get(list_container))
        .route("/metric/pod", get(list_pod))
        .route("/metric/scenario", get(list_scenario))
}

pub async fn list_image() -> Json<NewImageList> {
    let s = common::etcd::get("metric/image").await.unwrap_or_default();
    let image: NewImageList = serde_json::from_str(&s).unwrap_or_default();
    Json(image)
}

pub async fn list_container() -> Json<NewContainerList> {
    let s = common::etcd::get("metric/container")
        .await
        .unwrap_or_default();
    let container: NewContainerList = serde_json::from_str(&s).unwrap_or_default();
    Json(container)
}

pub async fn list_pod() -> Json<NewPodList> {
    let s = common::etcd::get("metric/pod").await.unwrap_or_default();
    let pod: NewPodList = serde_json::from_str(&s).unwrap_or_default();
    Json(pod)
}

pub async fn list_scenario() -> Json<Vec<Scenario>> {
    let (mut k, mut v) = common::etcd::get_all("scenario").await.unwrap_or_default();

    let mut scenarios: Vec<Scenario> = Vec::new();
    for _ in 0..k.len() {
        let key = k.pop().unwrap();
        let split: Vec<&str> = key.split('/').collect();

        let name = split.get(1).unwrap().to_string();
        let status = v.pop().unwrap_or_default();
        let condition = common::etcd::get(&format!("scenario/{name}/conditions"))
            .await
            .unwrap();
        let action = common::etcd::get(&format!("scenario/{name}/actions"))
            .await
            .unwrap();
        scenarios.push(Scenario {
            name,
            status,
            condition,
            action,
        });
    }
    Json(scenarios)
}

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct Scenario {
    pub name: String,
    pub status: String,
    pub condition: String,
    pub action: String,
}
