mod grpc;

use common::apiserver::request_connection_server::RequestConnectionServer;
use common::apiserver::update_workload_connection_server::UpdateWorkloadConnectionServer;
use tonic::transport::Server;

#[tokio::main]
async fn main() {
    let addr = common::apiserver::API_SERVER_OPEN.parse().unwrap();
    let request_server = grpc::receiver::request_handler::GrpcRequestServer::default();
    let update_server = grpc::receiver::update_workload_handler::GrpcUpdateServer::default();

    println!("Piccolod api-server listening on {}", addr);

    let _ = Server::builder()
        .add_service(RequestConnectionServer::new(request_server))
        .add_service(UpdateWorkloadConnectionServer::new(update_server))
        .serve(addr)
        .await;
}
