use clap::Parser;
use std::path::PathBuf;

use crate::definition::Pod;
mod cli_parser;
mod definition;
mod file_handler;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli_parser::Arguments::parse();
    let yaml_path = PathBuf::from(&args.path);

    let contents = file_handler::read_file(&yaml_path)?;
    let pod: Pod = serde_yaml::from_str(&contents)?;
    let output_yaml: String = serde_yaml::to_string(&pod)?;
    let _ = file_handler::create_parsed_file(&yaml_path, output_yaml)?;
    Ok(())
}
