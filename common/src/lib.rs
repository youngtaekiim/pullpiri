pub const HOST_IP: &str = "10.159.57.33";

pub mod apiserver {
    pub use api::proto::apiserver::*;
    pub const API_SERVER_OPEN: &str = const_format::concatcp!(crate::HOST_IP, ":50101");
    pub const API_SERVER_CONNECT: &str =
        const_format::concatcp!("http://", crate::HOST_IP, ":50101");

    // Following enums are defined in api::proto::apiserver module.
    pub enum UpdateMethod {
        Start = 0,
        Stop = 1,
        Restart = 2,
        Reload = 3,
        Enable = 4,
        Disable = 5,
    }
    pub enum ControllerCommand {
        ListNode = 0,
        DaemonReload = 1,
    }
    pub enum NodeCommand {
        ListUnit = 0,
    }

    pub fn get_controller_command(cmd: ControllerCommand) -> ToServer {
        ToServer {
            to_server_content: Some(to_server::ToServerContent::Request(request::Request {
                request_content: Some(request::request::RequestContent::ControllerRequest(
                    request::ControllerRequest {
                        controller_command: match cmd {
                            ControllerCommand::ListNode => {
                                request::ControllerCommand::ListNode.into()
                            }
                            ControllerCommand::DaemonReload => {
                                request::ControllerCommand::DaemonReload.into()
                            }
                        },
                    },
                )),
            })),
        }
    }

    pub fn get_node_command(cmd: NodeCommand, node_name: &str) -> ToServer {
        ToServer {
            to_server_content: Some(to_server::ToServerContent::Request(request::Request {
                request_content: Some(request::request::RequestContent::NodeRequest(
                    request::NodeRequest {
                        node_command: match cmd {
                            NodeCommand::ListUnit => request::NodeCommand::ListUnit.into(),
                        },
                        node_name: node_name.to_owned(),
                    },
                )),
            })),
        }
    }

    pub fn get_unit_command(cmd: UpdateMethod, node_name: &str, unit_name: &str) -> ToServer {
        ToServer {
            to_server_content: Some(to_server::ToServerContent::UpdateWorkload(
                updateworkload::UpdateWorkload {
                    update_method: match cmd {
                        UpdateMethod::Start => updateworkload::UpdateMethod::Start.into(),
                        UpdateMethod::Stop => updateworkload::UpdateMethod::Stop.into(),
                        UpdateMethod::Restart => updateworkload::UpdateMethod::Restart.into(),
                        UpdateMethod::Reload => updateworkload::UpdateMethod::Reload.into(),
                        UpdateMethod::Enable => updateworkload::UpdateMethod::Enable.into(),
                        UpdateMethod::Disable => updateworkload::UpdateMethod::Disable.into(),
                    },
                    node_name: node_name.to_owned(),
                    unit_name: unit_name.to_owned(),
                },
            )),
        }
    }
}

pub mod statemanager {
    pub use api::proto::statemanager::*;
    pub const STATE_MANAGER_OPEN: &str = const_format::concatcp!(crate::HOST_IP, ":50010");
    pub const STATE_MANAGER_CONNECT: &str =
        const_format::concatcp!("http://", crate::HOST_IP, ":50010");
}

pub mod etcd {
    pub const ETCD_ENDPOINT: &str = const_format::concatcp!(crate::HOST_IP, ":2379");
    pub const LISTEN_PEER_URLS: &str = const_format::concatcp!("http://", crate::HOST_IP, ":2380");
    pub const LISTEN_CLIENT_URLS: &str =
        const_format::concatcp!("http://", crate::HOST_IP, ":2379");
    pub const ADVERTISE_CLIENT_URLS: &str =
        const_format::concatcp!("http://", crate::HOST_IP, ":2379");

    use etcd_client::{Client, Error};

    async fn get_client() -> Result<Client, Error> {
        Client::connect([ETCD_ENDPOINT], None).await
    }

    pub async fn put(key: &str, value: &str) -> Result<(), Error> {
        let mut client = get_client().await?;
        client.put(key, value, None).await?;
        Ok(())
    }

    pub async fn get(key: &str) -> Result<(), Error> {
        let mut client = get_client().await?;
        client.get(key, None).await?;
        Ok(())
    }

    pub async fn delete(key: &str) -> Result<(), Error> {
        let mut client = get_client().await?;
        client.delete(key, None).await?;
        Ok(())
    }
}
