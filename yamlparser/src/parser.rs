use common::apiserver;
use std::fs::File;
use std::io::prelude::*;
use yaml_rust::Yaml;
use yaml_rust::YamlEmitter;
use yaml_rust::YamlLoader;

use apiserver::scenario::Scenario;

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
    let condition = doc["spec"]["conditions"].as_hash().unwrap();

    println!("Metadata: {}", metadata);
    println!("Action: {:#?}", action);
    println!("Condition: {:#?}", condition);

    let mut out_str = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut out_str);
        emitter.compact(true);
        emitter.dump(&Yaml::Hash(condition.clone())).unwrap(); // Dump the YAML object to a String
    }

    let mut out_str2 = String::new();
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
