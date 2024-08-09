/*
 * SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
 * SPDX-License-Identifier: Apache-2.0
 */
mod decompress;
mod downloader;
mod file_handler;
pub mod parser;

use parser::{package, scenario};
use std::{error::Error, path::Path};

pub async fn handle_package(package_name: &str) -> Result<package::Package, Box<dyn Error>> {
    //url path
    let base_url = common::get_conf("DOC_REGISTRY");
    let full_url: String = format!("{}/packages/{}.tar", base_url, package_name);
    println!("full url : {}", full_url);

    //save path
    let save_path: String = common::get_conf("YAML_STORAGE");
    let full_save_path = format!("{}/packages/{}.tar", save_path, package_name);
    println!("full save path : {}", full_save_path);

    //download, decompress
    if !Path::new(&full_save_path).exists() {
        downloader::download(&full_url, &full_save_path).await?;
    }
    decompress::decompress(&full_save_path).await?;

    //parsing
    let parsing_path = format!("{}/packages/{}", save_path, package_name);
    package::package_parse(&parsing_path)
}

pub async fn handle_scenario(scenario_name: &str) -> Result<scenario::Scenario, Box<dyn Error>> {
    let base_url = common::get_conf("DOC_REGISTRY");
    let full_url = format!("{}/scenarios/{}.yaml", base_url, scenario_name);

    let save_path: String = common::get_conf("YAML_STORAGE");
    let full_save_path = format!("{}/scenarios/{}.yaml", save_path, scenario_name);

    if !Path::new(&full_save_path).exists() {
        downloader::download(&full_url, &full_save_path).await?;
    }
    scenario::scenario_parse(&full_save_path)
}

#[cfg(test)]
mod tests {
    use crate::{handle_package, handle_scenario};

    #[tokio::test]
    async fn package_parse() {
        let result = handle_package("package-version1").await;
        println!("{:#?}", result);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn scenario_parse() {
        let result = handle_scenario("launch-scenario").await;
        println!("{:#?}", result);
        assert!(result.is_ok());
    }
}
