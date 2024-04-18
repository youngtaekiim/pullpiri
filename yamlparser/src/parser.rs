use common::apiserver;
use std::fs::File;
use std::io::prelude::*;
use yaml_rust::YamlEmitter;
use yaml_rust::YamlLoader;

use apiserver::scenario::Scenario;

use crate::file_handler;
use crate::msg_sender::send_grpc_msg;

pub async fn parser(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // open YAML file
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let docs = YamlLoader::load_from_str(&contents)?;
    let doc = &docs[0];

    // Debugging
    //println!("{:#?}", doc);

    // Access parts of the document
    let name = doc["metadata"]["name"].as_str().unwrap();

    //let action = doc["spec"]["action"][0].as_hash().unwrap();
    //let condition = doc["spec"]["conditions"].as_hash().unwrap();
    let action = &doc["spec"]["action"][0];
    let condition = &doc["spec"]["conditions"];

    let operation = action["operation"].as_str().unwrap();
    let image = action["image"].as_str().unwrap();
    let version = image.split(':').collect::<Vec<&str>>().last().copied().unwrap();

    //for file name
    //let parts: Vec<&str> = image.split('/').collect();
    //let image_name_parts: Vec<&str> = parts[2].split(':').collect();
    //let file_name = image_name_parts[0];

    //println!("name: {}", name);
    //println!("Action: {:#?}", action);
    //println!("Condition: {:#?}", condition);

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
        emitter.compact(true);
        // Dump the YAML object to a String
        //emitter.dump(&Yaml::Hash(condition.as_hash().unwrap().clone()))?;
        emitter.dump(condition)?;
    }

    let mut str_action: String = String::new();
    {
        let mut emitter2 = YamlEmitter::new(&mut str_action);
        emitter2.compact(true);
        // Dump the YAML object to a String
        //emitter2.dump(&Yaml::Hash(action.as_hash().unwrap().clone()))?;
        emitter2.dump(action)?;
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
