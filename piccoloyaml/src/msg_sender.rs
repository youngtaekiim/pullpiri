use common::apiserver::request_connection_client::RequestConnectionClient;
use common::apiserver::{request::Request, Response};

pub async fn send_request_msg(send: Request) -> Result<tonic::Response<Response>, tonic::Status> {
    println!("sending msg - '{:?}'\n", send);

    let mut client = RequestConnectionClient::connect(common::apiserver::API_SERVER_CONNECT)
        .await
        .unwrap_or_else(|err| {
            println!("FAIL - {}\ncannot connect to gRPC server", err);
            std::process::exit(1);
        });

    client.send(tonic::Request::new(send)).await
}
