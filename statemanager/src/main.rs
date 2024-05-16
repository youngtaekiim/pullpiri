/*
 * SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
 * SPDX-License-Identifier: Apache-2.0
 */

mod grpc_server;
mod method_bluechi;

use crate::grpc_server::StateManagerGrpcServer;
use common::statemanager::connection_server::ConnectionServer;
use tonic::transport::Server;

#[tokio::main]
async fn main() {
    let addr = common::statemanager::STATE_MANAGER_OPEN
        .parse()
        .expect("statemanager address parsing error");
    let state_manager_grpc_server = StateManagerGrpcServer::default();

    println!("Piccolod api-server listening on {}", addr);

    let _ = Server::builder()
        .add_service(ConnectionServer::new(state_manager_grpc_server))
        .serve(addr)
        .await;
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_parsing() {
        let result =
            crate::grpc_server::make_action_for_scenario("scenario/version-display/action").await;
        println!("{:?}", result);
        assert!(result.is_ok());
    }
}
