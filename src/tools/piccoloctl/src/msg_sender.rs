/*
 * SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
 * SPDX-License-Identifier: Apache-2.0
 */

use common::apiserver::request_connection_client::RequestConnectionClient;
use common::apiserver::update_workload_connection_client::UpdateWorkloadConnectionClient;
use common::apiserver::{request::Request, updateworkload::UpdateWorkload, Response};

pub async fn send_request_msg(send: Request) -> Result<tonic::Response<Response>, tonic::Status> {
    println!("sending msg - '{:?}'\n", send);

    let mut client =
        match RequestConnectionClient::connect(common::apiserver::connect_server()).await {
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

pub async fn send_update_msg(
    send: UpdateWorkload,
) -> Result<tonic::Response<Response>, tonic::Status> {
    println!("sending msg - '{:?}'\n", send);

    let mut client =
        match UpdateWorkloadConnectionClient::connect(common::apiserver::connect_server()).await {
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
