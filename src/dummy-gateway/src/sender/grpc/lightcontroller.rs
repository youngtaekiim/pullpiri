use common::dummylightcontroller;

#[allow(dead_code)]
pub async fn send(
    onoff: bool,
) -> Result<tonic::Response<dummylightcontroller::Reply>, tonic::Status> {
    println!("sending policy (light on)\n");

    let mut client = match dummylightcontroller::dummy_light_controller_client::DummyLightControllerClient::connect(
        dummylightcontroller::connect_server(),
    )
    .await
    {
        Ok(c) => c,
        Err(_) => {
            return Err(tonic::Status::new(
                tonic::Code::Unavailable,
                "cannot connect dummy-light-controller",
            ))
        }
    };

    client
        .request_event(tonic::Request::new(dummylightcontroller::PolicyLightOn {
            light_on: onoff,
        }))
        .await
}
