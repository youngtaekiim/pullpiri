use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use common::spec::package;

pub struct Package {
    pub name: String,
    pub models: String,
    pub network: String,
    pub volume: String,
}

pub fn package_parse(path: &str) -> Result<Package, Box<dyn std::error::Error>> {
    let package_yaml = format!("{}/package.yaml", path);
    let mut f = File::open(package_yaml)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let package: package::Package = serde_yaml::from_str(&contents)?;
    
    let models_name: Vec<String> = package.get_models();

    for model_name in models_name {
        let model = model_parse(path, &model_name);
        let volume_mount: Option<Vec<common::spec::workload::podspec::VolumeMount>> = volume_parse(path);
        
    }

    let network = network_parse(path);
    let volume = volume_parse(path);
    let volume_mount = volume. 

    //To Do
    //merge model, network, volume
    //old_file_handler::perform(&name, actions)?;

    Ok(Package {
        name,
        conditions: serde_yaml::to_string(conditions)?,
        actions: serde_yaml::to_string(actions)?,
        targets: serde_yaml::to_string(targets)?,
        scene: serde_yaml::to_string(&scene)?,
    })

}

fn model_parse(path: &str, name: &str) -> Result<package::model::Model, Box<dyn std::error::Error>> {
    let model_yaml = format!("{}/models/{}.yaml", path, name);
    let mut f = File::open(model_yaml)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let model: package::model::Model = serde_yaml::from_str(&contents)?;

    Ok(model)
}

fn network_parse(path: &str) -> Result<package::network::Network, Box<dyn std::error::Error>> {
    let network_yaml = format!("{}/networks/network.yaml", path);
    let mut f = File::open(network_yaml)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let network: package::network::Network = serde_yaml::from_str(&contents)?;

    Ok(network)
}

fn volume_parse(path: &str) -> Result<package::volume::Volume, Box<dyn std::error::Error>> {
    let volume_yaml = format!("{}/volumes/volume.yaml", path);
    let mut f = File::open(volume_yaml)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let volume: package::volume::Volume = serde_yaml::from_str(&contents)?;

    Ok(volume)
}