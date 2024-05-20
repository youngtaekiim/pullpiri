/*
 * SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
 * SPDX-License-Identifier: Apache-2.0
 */

pub mod apiserver;
pub mod etcd;
pub mod gateway;
pub mod spec;
pub mod statemanager;
pub mod yamlparser;

pub mod constants {
    pub use api::proto::constants::*;
}

pub const YAML_STORAGE: &str = "/root/piccolo_yaml/";

pub fn get_ip() -> String {
    let conf = config::Config::builder()
        .add_source(config::File::with_name("piccolo"))
        .build()
        .unwrap();
    conf.get_string("HOST_IP").unwrap()
}
