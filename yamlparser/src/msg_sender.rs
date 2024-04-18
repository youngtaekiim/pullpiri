use common::apiserver::scenario_connection_client::ScenarioConnectionClient;
use common::apiserver::{scenario::Scenario, Response};

pub async fn send_grpc_msg(send: Scenario) -> Result<tonic::Response<Response>, tonic::Status> {
    println!("sending msg - '{:?}'\n", send);

    let mut client =
        match ScenarioConnectionClient::connect(common::apiserver::API_SERVER_CONNECT).await {
            Ok(c) => c,
            Err(e) => return Err(tonic::Status::new(tonic::Code::Unavailable, e.to_string())),
        };

    client.send(tonic::Request::new(send)).await
}
