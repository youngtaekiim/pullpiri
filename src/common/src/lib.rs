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

use std::sync::OnceLock;
static CONFIG: OnceLock<config::Config> = OnceLock::new();

fn init_conf() -> config::Config {
    config::Config::builder()
        .add_source(config::File::with_name("piccolo"))
        .build()
        .unwrap()
}

pub fn get_conf(key: &str) -> String {
    let conf = CONFIG.get_or_init(init_conf);
    conf.get_string(key).unwrap()
}
