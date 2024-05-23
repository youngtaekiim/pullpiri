/*
 * SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
 * SPDX-License-Identifier: Apache-2.0
 */

use common::apiserver::scenario_connection_client::ScenarioConnectionClient;
use common::apiserver::{scenario::Scenario, Response};

pub async fn send_grpc_msg(send: Scenario) -> Result<tonic::Response<Response>, tonic::Status> {
    println!("sending msg - '{:?}'\n", send);

    let mut client =
        match ScenarioConnectionClient::connect(common::apiserver::connect_server()).await {
            Ok(c) => c,
            Err(_) => {
                return Err(tonic::Status::new(
                    tonic::Code::Unavailable,
                    "cannot connect api-server",
                ))
            }
        };

    client.send(tonic::Request::new(send)).await
}
