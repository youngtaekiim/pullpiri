use common::apiserver::scenario_connection_client::ScenarioConnectionClient;
use common::apiserver::{scenario::Scenario, Response};

pub async fn send_grpc_msg(send: Scenario) -> Result<tonic::Response<Response>, tonic::Status> {
    println!("sending msg - '{:?}'\n", send);

    let mut client = ScenarioConnectionClient::connect(common::apiserver::API_SERVER_CONNECT)
        .await
        .unwrap_or_else(|err| {
            println!("FAIL - {}\ncannot connect to gRPC server", err);
            std::process::exit(1);
        });

    client.send(tonic::Request::new(send)).await
}
