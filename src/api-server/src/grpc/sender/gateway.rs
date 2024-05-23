/*
 * SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
 * SPDX-License-Identifier: Apache-2.0
 */

use common::apiserver::scenario::Scenario;
use common::etcd;
use common::gateway;

pub async fn send_msg_to_gateway(
    scenario: Scenario,
) -> Result<tonic::Response<gateway::Reply>, tonic::Status> {
    if let Err(e) = write_etcd(&scenario).await {
        return Err(tonic::Status::new(tonic::Code::Unavailable, e.to_string()));
    }

    let mut client =
        match gateway::piccolo_gateway_service_client::PiccoloGatewayServiceClient::connect(
            gateway::connect_server(),
        )
        .await
        {
            Ok(c) => c,
            Err(_) => {
                return Err(tonic::Status::new(
                    tonic::Code::Unavailable,
                    "cannot connect gateway",
                ))
            }
        };

    let event_name = gateway::EventName {
        id: gateway::FuncId::Enable.into(),
        name: format!("scenario/{}", &scenario.name),
        target: gateway::Target::StateManager.into(),
    };

    client.request_event(tonic::Request::new(event_name)).await
}

async fn write_etcd(scenario: &Scenario) -> Result<(), etcd::Error> {
    let name = &scenario.name;
    let conditions = &scenario.conditions;
    let actions = &scenario.actions;

    let condition_key = format!("scenario/{}/conditions", name);
    let action_key = format!("scenario/{}/action", name);

    etcd::put(&action_key, actions).await?;
    etcd::put(&condition_key, conditions).await?;

    Ok(())
}
