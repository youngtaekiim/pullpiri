use common::statemanager;

pub async fn send_msg_to_statemanager(
    msg: &str,
) -> Result<tonic::Response<statemanager::SendResponse>, tonic::Status> {
    println!("sending msg - '{}'\n", msg);

    let mut client = match statemanager::connection_client::ConnectionClient::connect(
        statemanager::STATE_MANAGER_CONNECT,
    )
    .await
    {
        Ok(c) => c,
        Err(e) => return Err(tonic::Status::new(tonic::Code::Unavailable, e.to_string())),
    };

    client
        .send(tonic::Request::new(statemanager::SendRequest {
            from: common::constants::PiccoloModuleName::Apiserver.into(),
            request: msg.to_owned(),
        }))
        .await
}
