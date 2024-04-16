use common::apiserver::scenario::Scenario;
use common::etcd;
use common::gateway;

pub async fn send_msg_to_gateway(
    scenario: Scenario,
) -> Result<tonic::Response<gateway::Reply>, tonic::Status> {
    let _ = etcd::put(&("scenario/".to_owned() + &scenario.name), "asdf").await;

    if let Err(e) = write_etcd(&scenario).await {
        return Err(tonic::Status::new(tonic::Code::Unavailable, e.to_string()));
    }

    let mut client =
        match gateway::piccolo_gateway_service_client::PiccoloGatewayServiceClient::connect(
            gateway::GATEWAY_CONNECT,
        )
        .await
        {
            Ok(c) => c,
            Err(e) => return Err(tonic::Status::new(tonic::Code::Unavailable, e.to_string())),
        };

    let event_name = gateway::EventName {
        is_enable: true,
        name: format!("scenario/{}", &scenario.name),
        target: gateway::Target::StateManager.into(),
    };

    client.request_event(tonic::Request::new(event_name)).await
}

async fn write_etcd(scenario: &Scenario) -> Result<(), etcd::Error> {
    let name = &scenario.name;
    let conditions = &scenario.conditions;
    let actions = &scenario.actions;

    let condition_key = format!("scenario/{}", name);
    let action_key = format!("scenario/action/{}", name);

    let condition_value = format!(
        "category: PiccoloEvent\naction: {}\n{}",
        action_key, conditions
    );
    let action_value = actions;

    etcd::put(&action_key, &action_value).await?;
    etcd::put(&condition_key, &condition_value).await?;

    Ok(())
}
