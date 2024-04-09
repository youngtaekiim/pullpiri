use common::etcd;
use common::statemanager;

pub async fn send_msg_to_statemanager(
    msg: &str,
) -> Result<tonic::Response<statemanager::SendResponse>, tonic::Status> {
    println!("sending msg - '{}'\n", msg);
    let _ = etcd::put("asdf", "asdf").await;
    let _ = etcd::get("asdf").await;
    let _ = etcd::delete("asdf").await;

    let mut client = statemanager::connection_client::ConnectionClient::connect(
        statemanager::STATE_MANAGER_CONNECT,
    )
    .await
    .unwrap_or_else(|err| {
        println!("FAIL - {}\ncannot connect to gRPC server", err);
        std::process::exit(1);
    });

    client
        .send(tonic::Request::new(statemanager::SendRequest {
            from: "api-server".to_owned(),
            request: msg.to_owned(),
        }))
        .await
}
