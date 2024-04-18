mod grpc_server;
mod method_bluechi;

use crate::grpc_server::StateManagerGrpcServer;
use common::statemanager::connection_server::ConnectionServer;
use tonic::transport::Server;

#[tokio::main]
async fn main() {
    // for test
    match grpc_server::make_action_for_scenario("scenario/action/test").await {
        Ok(_) => println!("Good parsing job"),
        Err(e) => println!("{:?}", e.to_string()),
    };
    // for test

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
