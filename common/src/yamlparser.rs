/*
 * SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
 * SPDX-License-Identifier: Apache-2.0
 */

pub use api::proto::yamlparser::*;

pub fn open_server() -> String {
    format!("{}:47004", crate::get_ip())
}

pub fn connect_server() -> String {
    format!("http://{}:47004", crate::get_ip())
}
