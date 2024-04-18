use common::apiserver;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use yaml_rust::YamlEmitter;
use yaml_rust::YamlLoader;

use apiserver::scenario::Scenario;

use crate::file_handler;
use crate::msg_sender::send_grpc_msg;

pub async fn parser(path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    // open YAML file
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let docs = YamlLoader::load_from_str(&contents)?;
    let doc = &docs[0];

    // Debugging
    //println!("{:#?}", doc);

    // Access parts of the document
    let name = doc["metadata"]["name"]
        .as_str()
        .ok_or("metadata-name is None")?;
    let action = &doc["spec"]["action"][0];
    let condition = &doc["spec"]["conditions"];

    let operation = action["operation"]
        .as_str()
        .ok_or("action-operation is None")?;
    let image = action["image"].as_str().ok_or("action-image is None")?;
    let version = image
        .split(':')
        .collect::<Vec<&str>>()
        .last()
        .copied()
        .ok_or("cannot find version")?;

    match operation {
        "update" => {
            let _ = file_handler::update_yaml_file(image, name, version);
        }
        "rollback" => {}
        _ => {}
    }

    let mut str_condition = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut str_condition);
        emitter.dump(condition)?;
    }

    let mut str_action: String = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut str_action);
        emitter.dump(action)?;
    }

    let scenario: Scenario = Scenario {
        name: name.to_string(),
        conditions: str_condition,
        actions: str_action,
    };

    match send_grpc_msg(scenario).await {
        Ok(_) => Ok(()),
        Err(_) => Err("asd".into()),
    }
}
