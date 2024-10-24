/*
 * SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
 * SPDX-License-Identifier: Apache-2.0
 */
mod file_handler;
pub mod parser;

use parser::{package, scenario};
use std::{error::Error, path::Path};

pub async fn handle_package(package_name: &str) -> Result<package::Package, Box<dyn Error>> {
    //url path
    let full_url: String = format!(
        "{}/packages/{}.tar",
        common::get_config().doc_registry,
        package_name
    );
    println!("full url : {}", full_url);

    //save path
    let full_save_path = format!(
        "{}/packages/{}.tar",
        common::get_config().yaml_storage,
        package_name
    );
    println!("full save path : {}", full_save_path);

    //download, decompress
    if !Path::new(&full_save_path).exists() {
        file_handler::download(&full_url, &full_save_path).await?;
    }
    file_handler::extract(&full_save_path)?;

    //parsing
    let parsing_path = format!(
        "{}/packages/{}",
        common::get_config().yaml_storage,
        package_name
    );
    package::parse(&parsing_path)
}

pub async fn handle_scenario(scenario_name: &str) -> Result<scenario::Scenario, Box<dyn Error>> {
    let full_url = format!(
        "{}/scenarios/{}.yaml",
        common::get_config().doc_registry,
        scenario_name
    );
    let full_save_path = format!(
        "{}/scenarios/{}.yaml",
        common::get_config().yaml_storage,
        scenario_name
    );

    if !Path::new(&full_save_path).exists() {
        file_handler::download(&full_url, &full_save_path).await?;
    }
    scenario::parse(&full_save_path)
}

#[cfg(test)]
mod tests {
    use crate::file_handler;

    #[tokio::test]
    async fn downloading() {
        let url = "http://sdv.lge.com:9001/piccolo/resources/packages/version-cli-1.tar";
        let path = "/root/Music/test.tar";
        let result = file_handler::download(url, path).await;
        println!("{:?}", result);
        assert!(result.is_ok());
    }
}
