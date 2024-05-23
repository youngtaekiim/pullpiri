/*
 * SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
 * SPDX-License-Identifier: Apache-2.0
 */

use std::io::{Result, Write};
use std::path::PathBuf;
use std::{fs::File, io::Read};

pub fn read_file(file_path: &PathBuf) -> Result<String> {
    let mut file = File::open(file_path)?;
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn create_parsed_file(file_path: &PathBuf, yaml_contents: String) -> Result<()> {
    let mut output_path = PathBuf::from(file_path);
    output_path.set_file_name("parsed_definition.yaml");
    let mut output_file = File::create(output_path)?;
    output_file.write_all(yaml_contents.as_bytes())?;
    Ok(())
}
