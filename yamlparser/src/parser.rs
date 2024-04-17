use common::apiserver;
use std::fs::File;
use std::io::prelude::*;
use yaml_rust::Yaml;
use yaml_rust::YamlEmitter;
use yaml_rust::YamlLoader;

use apiserver::scenario::Scenario;

use crate::file_handler;
use crate::msg_sender::send_grpc_msg;

pub async fn parser(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    // YAML 파일 열기
    println!("START");
    let mut file = File::open(name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let docs = YamlLoader::load_from_str(&contents).unwrap();

    // Multi-document support, doc is a yaml::Yaml
    let doc = &docs[0];

    // Debugging
    println!("{:#?}", doc);

    // Access parts of the document
    let metadata = doc["metadata"]["name"].as_str().unwrap();
    let action = doc["spec"]["action"][0].as_hash().unwrap();
    let image = doc["spec"]["action"][0]["image"].as_str().unwrap();
    let condition = doc["spec"]["conditions"].as_hash().unwrap();
    let operation = doc["spec"]["action"][0]["operation"].as_str().unwrap();
    let version = image.split(':').collect::<Vec<&str>>()[1];

    //for file name
    let parts: Vec<&str> = image.split('/').collect();
    let image_name_parts: Vec<&str> = parts[2].split(':').collect();
    let file_name = image_name_parts[0];

    println!("Metadata: {}", metadata);
    println!("Action: {:#?}", action);
    println!("Condition: {:#?}", condition);

    match operation {
        "update" => {
            let _ = file_handler::update_yaml_file(image, file_name, version);
        }
        "rollback" => {}
        _ => {}
    }

    let mut out_str = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut out_str);
        emitter.compact(true);
        emitter.dump(&Yaml::Hash(condition.clone())).unwrap(); // Dump the YAML object to a String
    }

    let mut out_str2: String = String::new();
    {
        let mut emitter2 = YamlEmitter::new(&mut out_str2);
        emitter2.compact(true);
        emitter2.dump(&Yaml::Hash(action.clone())).unwrap(); // Dump the YAML object to a String
    }

    let scenario: Scenario = Scenario {
        name: metadata.to_string(),
        conditions: out_str,
        actions: out_str2,
    };

    match send_grpc_msg(scenario).await {
        Ok(_) => Ok(()),
        Err(_) => Err("asd".into()),
    }
}
