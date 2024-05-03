use crate::method_bluechi::{method_controller, method_node, method_unit};
use common::etcd;
use common::statemanager::connection_server::Connection;
use common::statemanager::{SendRequest, SendResponse};
use std::io::{Error, ErrorKind};

const SYSTEMD_PATH: &str = "/etc/containers/systemd/";

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
    let cmd: Vec<&str> = msg.split('/').collect();

    match cmd.len() {
        1 => method_controller::handle_cmd(cmd),
        2 => method_node::handle_cmd(cmd),
        3 => method_unit::handle_cmd(cmd),
        _ => Err("support only 1 ~ 3 parameters".into()),
    }
}

pub async fn make_action_for_scenario(key: &str) -> Result<String, Box<dyn std::error::Error>> {
    let value = etcd::get(key).await?;
    /*let value = r#"---
    operation: update
    image: "sdv.lge.com/library/passive-redundant-pong:0.2""#;*/

    let action: common::Action = serde_yaml::from_str(&value)?;
    let name = key.split('/').collect::<Vec<&str>>()[1];
    let operation = &*action.get_operation();
    let image = action.get_image();

    println!(
        "name : {}\noperation : {}\nimage: {}\n",
        name, operation, image
    );

    match operation {
        "deploy" => {
            delete_symlink_and_reload(name)?;
            make_and_start_new_symlink(name, &image)?;
        }
        "update" | "rollback" => {
            delete_symlink_and_reload(name)?;
            make_and_start_new_symlink(name, &image)?;
        }
        _ => {
            return Err("not supported operation".into());
        }
    }

    Ok(format!("Done : {}\n", operation))
}

fn delete_symlink_and_reload(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let _ = method_unit::handle_cmd(vec!["STOP", "nuc-cent", &format!("{}.service", name)]);
    let kube_symlink_path = format!("{}{}.kube", SYSTEMD_PATH, name);
    let _ = std::fs::remove_file(kube_symlink_path);
    method_controller::handle_cmd(vec!["DAEMON_RELOAD"])?;
    Ok(())
}

fn make_and_start_new_symlink(name: &str, image: &str) -> Result<(), Box<dyn std::error::Error>> {
    let version = image
        .split(':')
        .collect::<Vec<&str>>()
        .last()
        .copied()
        .ok_or(Error::new(ErrorKind::NotFound, "cannot find image version"))?;

    let original = format!("{0}{1}/{1}_{2}.kube", common::YAML_STORAGE, name, version);
    let link = format!("{}{}.kube", SYSTEMD_PATH, name);
    std::os::unix::fs::symlink(original, link)?;

    method_controller::handle_cmd(vec!["DAEMON_RELOAD"])?;
    method_unit::handle_cmd(vec!["START", "nuc-cent", &format!("{}.service", name)])?;

    Ok(())
}
