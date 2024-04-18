mod file_handler;
mod grpc_msg_receiver;
mod msg_sender;
mod parser;

use common::yamlparser::connection_server::ConnectionServer;
use grpc_msg_receiver::YamlparserGrpcServer;
use tonic::transport::Server;

#[tokio::main]
async fn main() {
    let addr = common::yamlparser::YAML_PARSER_OPEN.parse().unwrap();
    let piccoloyaml_grpc_server = YamlparserGrpcServer::default();

    println!("yaml grpc server listening on {}", addr);
    let _ = Server::builder()
        .add_service(ConnectionServer::new(piccoloyaml_grpc_server))
        .serve(addr)
        .await;
}
