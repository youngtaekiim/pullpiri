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

pub const HOST_IP: &str = "10.157.19.218";
pub const YAML_STORAGE: &str = "/root/piccolo_yaml/";

