use crate::file_handler;
use common::apiserver::scenario::Scenario;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

pub async fn parse(path: &PathBuf) -> Result<Scenario, Box<dyn std::error::Error>> {
    let mut f = File::open(path)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let yaml_map: serde_yaml::Mapping = serde_yaml::from_str(&contents)?;

    let name = yaml_map["metadata"]["name"]
        .as_str()
        .ok_or("None - metadata.name")?;
    let action = &yaml_map["spec"]["action"][0];
    let condition = &yaml_map["spec"]["conditions"];

    let operation = action["operation"]
        .as_str()
        .ok_or("None - action.operation")?;
    let image = action["image"].as_str().ok_or("None - action.image")?;
    let version = image
        .split(':')
        .collect::<Vec<&str>>()
        .last()
        .copied()
        .ok_or("cannot find version")?;

    file_handler::perform(name, operation, image, version)?;

    Ok(Scenario {
        name: name.to_string(),
        conditions: serde_yaml::to_string(condition)?,
        actions: serde_yaml::to_string(action)?,
    })
}
