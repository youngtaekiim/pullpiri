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

use config::Config;

//pub const HOST_IP: &str = "10.157.19.218";
pub const YAML_STORAGE: &str = "/root/piccolo_yaml/";

pub fn get_ip() -> String {
    let conf = Config::builder()
        .add_source(config::File::with_name("piccolo"))
        .build()
        .unwrap();
    conf.get_string("HOST_IP").unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn config_works() {
        assert_eq!(crate::apiserver::open_server(), "10.157.19.218:47001");
    }
}