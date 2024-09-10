/*
 * SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
 * SPDX-License-Identifier: Apache-2.0
 */

use common::statemanager::{connect_server, connection_client::ConnectionClient, Action, Response};
use tonic::{Request, Status};

#[allow(dead_code)]
pub async fn send(msg: &str) -> Result<tonic::Response<Response>, Status> {
    println!("sending msg - '{}'\n", msg);
    let action = Action {
        action: msg.to_string(),
    };

    let mut client = ConnectionClient::connect(connect_server()).await.unwrap();
    client.send_action(Request::new(action)).await
}
