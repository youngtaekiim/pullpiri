/*
 * SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
 * SPDX-License-Identifier: Apache-2.0
 */

use common::apiserver::scenario_connection_client::ScenarioConnectionClient;
use common::apiserver::{scenario::Response, scenario::Scenario};

pub async fn send_msg_to_apiserver(
    send: Scenario,
) -> Result<tonic::Response<Response>, tonic::Status> {
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
