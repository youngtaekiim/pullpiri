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
                get_condition_key(&condition.get_operand_name()),
                capitalize_first_letter(&condition.get_value()),
            );
        }

        let mut metric_action = HashMap::new();
        let action_str = common::etcd::get(&format!("scenario/{name}/targets"))
            .await
            .unwrap();
        metric_action.insert(get_action_key(&action_str), get_action_value(&action_str));

        scenarios.push(ScenarioInfo {
            name: String::from(name),
            status,
            condition: metric_condition,
            action: metric_action,
        });
    }
    Json(scenarios)
}

fn get_condition_key(operand_name: &str) -> String {
    match operand_name {
        "gear" => String::from("Gear"),
        "daynight" => String::from("Daynight"),
        "sensor" => String::from("Proximity sensors"),
        "headlampstt" => String::from("Head Lamp"),
        "bttportstt" => String::from("Charger"),
        "trunkstt" => String::from("Trunk"),
        "leftwinstt" => String::from("Driver Window"),
        "rightwinstt" => String::from("Passenger Window"),
        "leftdoorstt" => String::from("Driver Door"),
        "rightdoorstt" => String::from("Passenger Door"),
        _ => String::new(),
    }
}

fn get_action_key(target_str: &str) -> String {
    if target_str.contains("performance") {
        String::from("Power")
    } else if target_str.contains("eco") {
        String::from("Eco")
    } else if target_str.contains("antipinch") {
        String::from("Anti pinch")
    } else if target_str.contains("headlamp") {
        String::from("Head Lamp")
    } else if target_str.contains("trunk") {
        String::from("Trunk")
    } else if target_str.contains("welcome") {
        String::from("Welcome")
    } else if target_str.contains("bttport") {
        String::from("Charger")
    } else if target_str.contains("leftdoor") {
        String::from("Driver Door")
    } else if target_str.contains("rightdoor") {
        String::from("Passenger Door")
    } else if target_str.contains("leftwin") {
        String::from("Driver Window")
    } else if target_str.contains("rightwin") {
        String::from("Passenger Window")
    } else {
        String::new()
    }
}

fn get_action_value(target_str: &str) -> String {
    if target_str.contains("bms") {
        String::from("ON")
    } else if target_str.contains("antipinch-enable") {
        String::from("Update")
    } else if target_str.contains("antipinch-disable") {
        String::from("Rollback")
    } else if target_str.contains("-open") {
        String::from("Open")
    } else if target_str.contains("-close") {
        String::from("Close")
    } else if target_str.contains("-on") {
        String::from("On")
    } else if target_str.contains("-off") {
        String::from("Off")
    } else {
        get_oem_value(target_str)
    }
}

fn get_oem_value(target_str: &str) -> String {
    if target_str.contains("-gm") {
        String::from("GM")
    } else if target_str.contains("-porsche") {
        String::from("Porsche")
    } else if target_str.contains("-kgmobility") {
        String::from("KGMobility")
    } else if target_str.contains("-toyota") {
        String::from("Toyota")
    } else if target_str.contains("-bmw") {
        String::from("BMW")
    } else if target_str.contains("-honda") {
        String::from("Honda")
    } else if target_str.contains("-jlr") {
        String::from("JLR")
    } else if target_str.contains("-audi") {
        String::from("Audi")
    } else if target_str.contains("-hyundai") {
        String::from("Hyundai")
    } else if target_str.contains("-kia") {
        String::from("Kia")
    } else if target_str.contains("-vw") {
        String::from("VW")
    } else if target_str.contains("-nissan") {
        String::from("Nissan")
    } else if target_str.contains("-renault") {
        String::from("Renault")
    } else {
        String::new()
    }
}

fn capitalize_first_letter(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

// fn capitalize_first_letter_only(s: &str) -> String {
//     let mut chars = s.chars();
//     match chars.next() {
//         Some(first) => first.to_uppercase().collect::<String>(),
//         None => String::new(),
//     }
// }

#[derive(serde::Deserialize, serde::Serialize, Default)]
struct ScenarioInfo {
    name: String,
    status: String,
    condition: HashMap<String, String>,
    action: HashMap<String, String>,
}
