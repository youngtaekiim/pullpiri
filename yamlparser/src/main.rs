mod file_handler;
mod msg_sender;
mod parser;
// use common::apiserver::{get_controller_command, ControllerCommand};
mod grpc_msg_receiver;
use crate::grpc_msg_receiver::YamlparserGrpcServer;

use common::yamlparser::connection_server::ConnectionServer;
use tonic::transport::Server;

#[tokio::main]
async fn main() {
    let _ =
        parser::parser("/root/work/projects-rust/piccolo-bluechi/bin/update-scenario.yaml").await;

    let addr = common::yamlparser::YAML_PARSER_OPEN.parse().unwrap();
    let piccoloyaml_grpc_server = YamlparserGrpcServer::default();
    //test for yaml parsing

    println!("yaml grpc server listening on {}", addr);
    let _ = Server::builder()
        .add_service(ConnectionServer::new(piccoloyaml_grpc_server))
        .serve(addr)
        .await;
}
