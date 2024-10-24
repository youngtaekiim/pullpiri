/*
 * SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
 * SPDX-License-Identifier: Apache-2.0
 */

use clap::Parser;
use std::path::PathBuf;

use common::spec::k8s::pod::Pod;
mod cli_parser;
mod file_handler;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli_parser::Arguments::parse();
    let yaml_path = PathBuf::from(&args.path);

    let contents = file_handler::read_file(&yaml_path)?;
    let pod: Pod = serde_yaml::from_str(&contents)?;
    let output_yaml: String = serde_yaml::to_string(&pod)?;
    file_handler::create_parsed_file(&yaml_path, output_yaml)?;
    Ok(())
}

/*
 * For testing,
 * cargo test -- --nocapture
*/
#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use serde_yaml;

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct ResourceScenario {
        pub name: String,
        condition: Condition,
        policy: Policy,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    struct Condition {
        name: Option<String>,
        criteria: Vec<Criterion>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    struct Policy {
        name: Option<String>,
        act: Vec<Act>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    struct Criterion {
        message: String,
        value: String,
        operand: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    struct Act {
        message: String,
        value: String,
    }

    #[test]
    fn make_yaml_string() {
        let cc1 = Criterion {
            message: "rt/piccolo/gear_state".to_string(),
            value: "driving".to_string(),
            operand: "equal".to_string(),
        };
        let cc2 = Criterion {
            message: "rt/piccolo/day".to_string(),
            value: "night".to_string(),
            operand: "equal".to_string(),
        };
        let c = Condition {
            name: None,
            criteria: vec![cc1, cc2],
        };
        let aa = Act {
            message: "rt/piccolo/light_on".to_string(),
            value: "true".to_string(),
        };
        let p = Policy {
            name: None,
            act: vec![aa],
        };

        let a = ResourceScenario {
            name: "fakename".to_string(),
            condition: c,
            policy: p,
        };

        println!("\n{:?}\n\n", a);

        let bb = serde_yaml::to_string(&a).unwrap();
        println!("{:#?}", bb);
    }
}
