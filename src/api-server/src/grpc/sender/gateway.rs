/*
 * SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
 * SPDX-License-Identifier: Apache-2.0
 */

use common::gateway;
use common::gateway::{Condition, Response};
use tonic::{Request, Status};

pub async fn send(condition: Condition) -> Result<tonic::Response<Response>, Status> {
    let mut client = match gateway::connection_client::ConnectionClient::connect(
        gateway::connect_server(),
    )
    .await
    {
        Ok(c) => c,
        Err(_) => {
            return Err(Status::new(
                tonic::Code::Unavailable,
                "cannot connect gateway",
            ))
        }
    };

    client.send_condition(Request::new(condition)).await
}
