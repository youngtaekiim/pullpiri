mod msg_sender;
mod file_handler;
mod parser;
// use common::apiserver::{get_controller_command, ControllerCommand};
mod grpc_msg_receiver;
use crate::grpc_msg_receiver::PiccoloyamlGrpcServer;

use common::piccoloyaml::connection_server::ConnectionServer;
use tonic::transport::Server;


#[tokio::main]
async fn main() {
    let addr = common::piccoloyaml::PICCOLOYAML_OPEN.parse().unwrap();
    let piccoloyaml_grpc_server = PiccoloyamlGrpcServer::default();
    println!("yaml grpc server listening on {}", addr);
    let _ = Server::builder()
    .add_service(ConnectionServer::new(piccoloyaml_grpc_server))
    .serve(addr)
    .await;
}
