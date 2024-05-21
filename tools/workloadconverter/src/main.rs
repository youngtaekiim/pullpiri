/*
 * SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
 * SPDX-License-Identifier: Apache-2.0
 */

use clap::Parser;
use std::path::PathBuf;

use common::spec::workload_spec::Pod;
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
