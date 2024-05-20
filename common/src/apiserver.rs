/*
 * SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
 * SPDX-License-Identifier: Apache-2.0
 */

pub use api::proto::apiserver::*;

pub fn open_server() -> String {
    format!("{}:47001", crate::get_ip())
}

pub fn connect_server() -> String {
    format!("http://{}:47001", crate::get_ip())
}

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

pub fn get_controller_command(cmd: ControllerCommand) -> request::Request {
    request::Request {
        request_content: Some(request::request::RequestContent::ControllerRequest(
            request::ControllerRequest {
                controller_command: match cmd {
                    ControllerCommand::ListNode => request::ControllerCommand::ListNode.into(),
                    ControllerCommand::DaemonReload => {
                        request::ControllerCommand::DaemonReload.into()
                    }
                },
            },
        )),
    }
}

pub fn get_node_command(cmd: NodeCommand, node_name: &str) -> request::Request {
    request::Request {
        request_content: Some(request::request::RequestContent::NodeRequest(
            request::NodeRequest {
                node_command: match cmd {
                    NodeCommand::ListUnit => request::NodeCommand::ListUnit.into(),
                },
                node_name: node_name.to_owned(),
            },
        )),
    }
}

pub fn get_unit_command(
    cmd: UpdateMethod,
    node_name: &str,
    unit_name: &str,
) -> updateworkload::UpdateWorkload {
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
    }
}
