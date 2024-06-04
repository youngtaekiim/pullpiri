/*
 * SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
 * SPDX-License-Identifier: Apache-2.0
 */

mod file_handler;
mod grpc;
mod parser;

use common::yamlparser::connection_server::ConnectionServer;
use grpc::receiver::scenario_handler::YamlparserGrpcServer;
use tonic::transport::Server;

#[tokio::main]
async fn main() {
    let addr = common::yamlparser::open_server()
        .parse()
        .expect("yamlparser address parsing error");
    let piccoloyaml_grpc_server = YamlparserGrpcServer::default();

    println!("yaml grpc server listening on {}", addr);
    let _ = Server::builder()
        .add_service(ConnectionServer::new(piccoloyaml_grpc_server))
        .serve(addr)
        .await;
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn parsing_update_scenario() {
        let path = std::path::PathBuf::from(
            "/root/work/projects-rust/piccolo/doc/examples/version-display/scenario/update-scenario.yaml",
        );

        let result = crate::parser::parse(&path).await;
        println!("{:#?}", result);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn parsing_rollback_scenario() {
        let path = std::path::PathBuf::from(
            "/root/work/projects-rust/piccolo/doc/examples/version-display/scenario/rollback-scenario.yaml",
        );

        let result = crate::parser::parse(&path).await;
        println!("{:#?}", result);
        assert!(result.is_ok());
    }
}
