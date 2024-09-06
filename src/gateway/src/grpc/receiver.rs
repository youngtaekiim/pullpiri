/*
 * SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
 * SPDX-License-Identifier: Apache-2.0
 */

use common::gateway::connection_server::Connection;
use common::gateway::{Condition, Response};
use tokio::sync::mpsc;

pub struct GrpcServer {
    pub grpc_msg_tx: mpsc::Sender<Condition>,
}

#[tonic::async_trait]
impl Connection for GrpcServer {
    async fn send_condition(
        &self,
        request: tonic::Request<Condition>,
    ) -> Result<tonic::Response<Response>, tonic::Status> {
        println!("Got a request from {:?}", request.remote_addr());
        let req = request.into_inner();
        //println!("req msg : {:#?}", req);
        let _ = self.grpc_msg_tx.send(req).await;

        Ok(tonic::Response::new(Response { resp: true.to_string() }))
    }
}
