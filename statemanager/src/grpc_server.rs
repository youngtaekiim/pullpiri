use crate::method_bluechi::{method_controller, method_node, method_unit};
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
            match send_dbus(&command).await {
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

async fn send_dbus(msg: &str) -> Result<String, Box<dyn std::error::Error>> {
    println!("recv msg: {}\n", msg);
    let cmd: Vec<&str> = msg.split("/").collect();

    match cmd.len() {
        1 => method_controller::handle_cmd(cmd),
        2 => method_node::handle_cmd(cmd),
        3 => method_unit::handle_cmd(cmd),
        _ => Err("support only 1 ~ 3 parameters".into()),
    }
}

pub async fn make_action_for_scenario(key: &str) -> Result<String, Box<dyn std::error::Error>> {
    // TODO - manage symbolic link
    let value = etcd::get(key).await?;
    /*let value = r#"---
    operation: update
    image: "sdv.lge.com/library/passive-redundant-pong:0.2""#;*/

    let docs = YamlLoader::load_from_str(&value)?;
    //let doc = &docs[0]["action"][0];
    let doc = &docs[0];

    let name = key
        .split('/')
        .collect::<Vec<&str>>()
        .last()
        .copied()
        .ok_or("name is None")?;
    let action = doc["operation"].as_str().ok_or("action is None")?;
    let image = doc["image"].as_str().ok_or("image is None")?;

    println!("name : {}\naction : {}\nimage: {}", name, action, image);
    Ok("".into())
}
