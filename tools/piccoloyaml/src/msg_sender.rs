use common::apiserver::request_connection_client::RequestConnectionClient;
use common::apiserver::{request::Request, Response};

pub async fn send_request_msg(send: Request) -> Result<tonic::Response<Response>, tonic::Status> {
    println!("sending msg - '{:?}'\n", send);

    let mut client =
        match RequestConnectionClient::connect(common::apiserver::API_SERVER_CONNECT).await {
            Ok(c) => c,
            Err(_) => {
                return Err(tonic::Status::new(
                    tonic::Code::Unavailable,
                    "cannot connect api-server",
                ))
            }
        };

    client.send(tonic::Request::new(send)).await
}
