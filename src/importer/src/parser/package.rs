use crate::file_handler;
use common::spec::package;
use serde::de::DeserializeOwned;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
pub struct Package {
    pub name: String,
    pub model_names: Vec<String>,
    pub models: Vec<String>,
    pub nodes: Vec<String>,
    pub networks: Vec<String>,
    pub volumes: Vec<String>,
}

pub fn parse(path: &str) -> Result<Package, Box<dyn std::error::Error>> {
    let package_yaml = format!("{}/package.yaml", path);
    let mut f = File::open(package_yaml)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let package: package::Package = serde_yaml::from_str(&contents)?;
    let name: String = package.get_name();

    let mut model_names: Vec<String> = Vec::new();
    let mut models: Vec<String> = Vec::new();
    let mut nodes: Vec<String> = Vec::new();
    let mut networks: Vec<String> = Vec::new();
    let mut volumes: Vec<String> = Vec::new();

    for m in package.get_models() {
        let mut model: package::model::Model = model_parse(path, &m.get_name()).unwrap();
        model_names.push(m.get_name());

        nodes.push(m.get_node());

        let network_yaml = m.get_resources().get_network();
        networks.push(serde_yaml::to_string(&network_parse(path, &network_yaml))?);

        let volume_yaml = m.get_resources().get_volume();
        volumes.push(serde_yaml::to_string(&volume_parse(path, &volume_yaml))?);

        if let Some(volume_spec) = &volume_parse(path, &volume_yaml) {
            model.spec.volumes.clone_from(volume_spec.get_volume());
        }

        models.push(serde_yaml::to_string(&model)?);

        //make kube,yaml file
        file_handler::perform(&m.get_name(), &serde_yaml::to_string(&model)?, &name)?;
    }

    file_handler::copy_to_remote_node(path)?;

    Ok(Package {
        name,
        model_names,
        models,
        nodes,
        networks,
        volumes,
    })
}

fn parse_yaml<T>(path: &str, name: &str, subdir: &str) -> Result<T, Box<dyn Error>>
where
    T: DeserializeOwned,
{
    let yaml_path = format!("{}/{}/{}.yaml", path, subdir, name);
    let mut f = File::open(yaml_path)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    let parsed: T = serde_yaml::from_str(&contents)?;
    Ok(parsed)
}

fn model_parse(path: &str, name: &str) -> Result<package::model::Model, Box<dyn Error>> {
    parse_yaml(path, name, "models")
}

fn network_parse(path: &str, name: &str) -> Option<package::network::NetworkSpec> {
    let network: package::network::Network = parse_yaml(path, name, "networks").unwrap();
    network.get_spec().clone()
}

fn volume_parse(path: &str, name: &str) -> Option<package::volume::VolumeSpec> {
    if let Ok(volume) = parse_yaml::<package::volume::Volume>(path, name, "volumes") {
        volume.get_spec().clone()
    } else {
        None
    }
}
