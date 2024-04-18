use crate::method_bluechi;
use common::etcd;
use common::statemanager::connection_server::Connection;
use common::statemanager::{SendRequest, SendResponse};
use yaml_rust::YamlLoader;

#[derive(Default)]
pub struct StateManagerGrpcServer {}

#[tonic::async_trait]
impl Connection for StateManagerGrpcServer {
    async fn send(
        &self,
        request: tonic::Request<SendRequest>,
    ) -> Result<tonic::Response<SendResponse>, tonic::Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let req = request.into_inner();
        let from = req.from;
        let command = req.request;
        println!("{}/{}", from, command);

        if from == common::constants::PiccoloModuleName::Apiserver.into() {
            match method_bluechi::send_dbus(&command).await {
                Ok(v) => Ok(tonic::Response::new(SendResponse { response: v })),
                Err(e) => Err(tonic::Status::new(tonic::Code::Unavailable, e.to_string())),
            }
        } else if from == common::constants::PiccoloModuleName::Gateway.into() {
            match make_action_for_scenario(&command).await {
                Ok(v) => Ok(tonic::Response::new(SendResponse { response: v })),
                Err(e) => Err(tonic::Status::new(tonic::Code::Unavailable, e.to_string())),
            }
        } else {
            Err(tonic::Status::new(
                tonic::Code::Unavailable,
                "unsupported 'from' module",
            ))
        }
    }
}

pub async fn make_action_for_scenario(key: &str) -> Result<String, Box<dyn std::error::Error>> {
    // TODO - manage symbolic link
    let value = etcd::get(key).await?;
    /*    let value =
    r#"---
operation: update
image: "sdv.lge.com/library/passive-redundant-pong:0.2""#;*/

    let docs = YamlLoader::load_from_str(&value).unwrap();
    let doc = &docs[0]["action"][0];

    let name = key.split('/').collect::<Vec<&str>>().last().copied().unwrap();
    let action = doc["operation"].as_str().unwrap();
    let image = doc["image"].as_str().unwrap();

    println!("name : {}\naction : {}\nimage: {}", name, action, image);
    Err(value.into())
}
