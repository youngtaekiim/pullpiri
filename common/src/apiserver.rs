/*
 * SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
 * SPDX-License-Identifier: Apache-2.0
 */

pub use api::proto::apiserver::*;
pub const API_SERVER_OPEN: &str = const_format::concatcp!(crate::HOST_IP, ":47001");
pub const API_SERVER_CONNECT: &str = const_format::concatcp!("http://", crate::HOST_IP, ":47001");

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
