/*
 * SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
 * SPDX-License-Identifier: Apache-2.0
 */

mod grpc;

use common::apiserver::scenario_connection_server::ScenarioConnectionServer;
use tonic::transport::Server;

#[tokio::main]
async fn main() {
    let addr = common::apiserver::open_server()
        .parse()
        .expect("api-server address parsing error");
    let scenario_server = grpc::receiver::scenario_handler::GrpcUpdateServer::default();

    println!("Piccolod api-server listening on {}", addr);

    let _ = Server::builder()
        .add_service(ScenarioConnectionServer::new(scenario_server))
        .serve(addr)
        .await;
}
