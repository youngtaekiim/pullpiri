/*
 * SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
 * SPDX-License-Identifier: Apache-2.0
 */

use std::collections::HashMap;

use common::apiserver::metric_connection_server::MetricConnection;
use common::apiserver::metric_notifier::{
    ContainerInfo, ContainerList, ImageList, PodContainerInfo, PodInfo, PodList,
};
use common::apiserver::Response;
use tonic::Request;

type GrpcResult = Result<tonic::Response<Response>, tonic::Status>;

#[derive(Default)]
pub struct GrpcMetricServer {}

#[tonic::async_trait]
impl MetricConnection for GrpcMetricServer {
    async fn send_image_list(&self, request: Request<ImageList>) -> GrpcResult {
        //println!("Got a request from {:?}", request.remote_addr());

        let image_list = request.into_inner();
        let node_name = &image_list.node_name;
        let etcd_key = format!("metric/image/{node_name}");
        let new_image_list = NewImageList::from(image_list);
        let json_string = serde_json::to_string(&new_image_list).unwrap();
        //println!("image\n{:#?}", j);
        let _ = common::etcd::put(&etcd_key, &json_string).await;

        Ok(tonic::Response::new(Response {
            resp: true.to_string(),
        }))
    }

    async fn send_container_list(&self, request: Request<ContainerList>) -> GrpcResult {
        //println!("Got a request from {:?}", request.remote_addr());

        let container_list = request.into_inner();
        let node_name = &container_list.node_name;
        let etcd_key = format!("metric/container/{node_name}");
        let new_container_list = NewContainerList::from(container_list);

        // fake metric - start
        let mut vec_new_container_info: Vec<NewContainerInfo> = new_container_list
            .containers
            .into_iter()
            .filter(|container| {
                container
                    .annotation
                    .contains_key("io.piccolo.annotations.package-name")
            })
            .collect();

        let mut vec_fake_container_info: Vec<NewContainerInfo> = vec![
            NewContainerInfo::make_dummy("DIMS", 1),
            NewContainerInfo::make_dummy("LKA", 2),
            NewContainerInfo::make_dummy("HUD", 3),
            NewContainerInfo::make_dummy("ChildLock", 4),
            NewContainerInfo::make_dummy("ADAS", 5),
            NewContainerInfo::make_dummy("MRNavi", 6),
            NewContainerInfo::make_dummy("ABS", 7),
            NewContainerInfo::make_dummy("SmartCruise", 8),
            NewContainerInfo::make_dummy("AutoHold", 9),
            NewContainerInfo::make_dummy("V2X Comm.", 10),
            NewContainerInfo::make_dummy("VisionRoof", 11),
            NewContainerInfo::make_dummy("ISG", 12),
        ];

        vec_new_container_info.append(&mut vec_fake_container_info);
        let renew_container_list = NewContainerList {
            containers: vec_new_container_info,
        };
        // fake metric - end

        //overwrite names with package-type
        /*
        for container in &mut renew_container_list.containers {
            let package_type = container.annotation.get("io.piccolo.annotations.package-type").unwrap();
            container.names = vec![package_type.clone()];
        }
        */

        // TODO : renew -> new
        let j = serde_json::to_string(&renew_container_list).unwrap();
        //println!("container\n{:#?}", j);

        let _ = common::etcd::put(&etcd_key, &j).await;

        Ok(tonic::Response::new(Response {
            resp: true.to_string(),
        }))
    }

    async fn send_pod_list(&self, request: Request<PodList>) -> GrpcResult {
        //println!("Got a request from {:?}", request.remote_addr());

        let pod_list = request.into_inner();
        let node_name = &pod_list.node_name;
        let etcd_key = format!("metric/pod/{node_name}");
        let new_pod_list = NewPodList::from(pod_list);
        let json_string = serde_json::to_string(&new_pod_list).unwrap();
        //println!("pod\n{:#?}", j);

        let _ = common::etcd::put(&etcd_key, &json_string).await;

        Ok(tonic::Response::new(Response {
            resp: true.to_string(),
        }))
    }
}

/*
 * Copied structure for applying serde trait
*/
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
pub struct NewImageList {
    pub images: Vec<String>,
}

impl From<ImageList> for NewImageList {
    fn from(value: ImageList) -> Self {
        NewImageList {
            images: value.images,
        }
    }
}

#[derive(Deserialize, Serialize, Default)]
pub struct NewContainerList {
    pub containers: Vec<NewContainerInfo>,
}

#[derive(Deserialize, Serialize)]
pub struct NewContainerInfo {
    pub id: String,
    pub names: Vec<String>,
    pub image: String,
    pub state: HashMap<String, String>,
    pub config: HashMap<String, String>,
    pub annotation: HashMap<String, String>,
}

impl From<ContainerList> for NewContainerList {
    fn from(value: ContainerList) -> Self {
        let nv = value
            .containers
            .into_iter()
            .map(NewContainerInfo::from)
            .collect::<Vec<NewContainerInfo>>();
        NewContainerList { containers: nv }
    }
}

impl From<ContainerInfo> for NewContainerInfo {
    fn from(value: ContainerInfo) -> Self {
        NewContainerInfo {
            id: value.id,
            names: value.names,
            image: value.image,
            state: value.state,
            config: value.config,
            annotation: value.annotation,
        }
    }
}

// fake metric - start
impl NewContainerInfo {
    fn make_dummy(name: &str, index: i32) -> Self {
        let dummy_str = format!("dummy{index}");
        let dummy_node = if index > 7 {
            String::from("ZONE")
        } else {
            String::from("HPC")
        };
        let dummy_anno = HashMap::<String, String>::from([
            (
                String::from("io.piccolo.annotations.package-name"),
                String::from("dummy"),
            ),
            (
                String::from("io.piccolo.annotations.package-type"),
                String::from(name),
            ),
        ]);

        NewContainerInfo {
            id: dummy_str.clone(),
            names: vec![String::from(name)],
            image: dummy_str.clone(),
            state: HashMap::<String, String>::from([(dummy_str.clone(), dummy_str.clone())]),
            config: HashMap::<String, String>::from([(String::from("Hostname"), dummy_node)]),
            annotation: dummy_anno,
        }
    }
}
// fake metric - end

#[derive(Deserialize, Serialize, Default)]
pub struct NewPodList {
    pub pods: Vec<NewPodInfo>,
}

#[derive(Deserialize, Serialize)]
pub struct NewPodInfo {
    pub id: String,
    pub name: String,
    pub containers: Vec<NewPodContainerInfo>,
    pub state: String,
    pub host_name: String,
    pub created: String,
}

#[derive(Deserialize, Serialize)]
pub struct NewPodContainerInfo {
    pub id: String,
    pub name: String,
    pub state: String,
}

impl From<PodList> for NewPodList {
    fn from(value: PodList) -> Self {
        let nv = value
            .pods
            .into_iter()
            .map(NewPodInfo::from)
            .collect::<Vec<NewPodInfo>>();
        NewPodList { pods: nv }
    }
}

impl From<PodInfo> for NewPodInfo {
    fn from(value: PodInfo) -> Self {
        let nv = value
            .containers
            .into_iter()
            .map(NewPodContainerInfo::from)
            .collect::<Vec<NewPodContainerInfo>>();

        NewPodInfo {
            id: value.id,
            name: value.name,
            containers: nv,
            state: value.state,
            host_name: value.host_name,
            created: value.created,
        }
    }
}

impl From<PodContainerInfo> for NewPodContainerInfo {
    fn from(value: PodContainerInfo) -> Self {
        NewPodContainerInfo {
            id: value.id,
            name: value.name,
            state: value.state,
        }
    }
}
