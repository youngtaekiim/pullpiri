// SPDX-License-Identifier: Apache-2.0

use common::gateway;
use importer::parser::package::Package;
use importer::parser::scenario::Scenario;

pub async fn handle_package_msg(p: Package) -> Result<(), Box<dyn std::error::Error>> {
    let key_origin = format!("package/{}", p.name);
    /*common::etcd::put(&format!("{key_origin}/models"), &p.models).await?;
    common::etcd::put(&format!("{key_origin}/network"), &p.network).await?;
    common::etcd::put(&format!("{key_origin}/volume"), &p.volume).await?;*/

    for i in 0..p.model_names.len() {
        common::etcd::put(
            &format!("{key_origin}/models/{}", p.model_names.get(i).unwrap()),
            p.models.get(i).unwrap(),
        )
        .await?;
        common::etcd::put(
            &format!("{key_origin}/nodes/{}", p.model_names.get(i).unwrap()),
            p.nodes.get(i).unwrap(),
        )
        .await?;
        common::etcd::put(
            &format!("{key_origin}/networks/{}", p.model_names.get(i).unwrap()),
            p.networks.get(i).unwrap(),
        )
        .await?;
        common::etcd::put(
            &format!("{key_origin}/volumes/{}", p.model_names.get(i).unwrap()),
            p.volumes.get(i).unwrap(),
        )
        .await?;
    }

    Ok(())
}

pub async fn handle_scenario_msg(s: Scenario) -> Result<(), Box<dyn std::error::Error>> {
    let key_origin = format!("scenario/{}", s.name);
    common::etcd::put(&format!("{key_origin}/actions"), &s.actions).await?;
    common::etcd::put(&format!("{key_origin}/conditions"), &s.conditions).await?;
    common::etcd::put(&format!("{key_origin}/targets"), &s.targets).await?;
    common::etcd::put(&format!("{key_origin}/full"), &s.scene).await?;

    let condition = gateway::Condition {
        name: format!("scenario/{}", &s.name),
    };

    crate::grpc::sender::gateway::send(condition).await?;

    Ok(())
}
