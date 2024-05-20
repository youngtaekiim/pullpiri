/*
 * SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
 * SPDX-License-Identifier: Apache-2.0
 */

pub use api::proto::statemanager::*;

pub fn open_server() -> String {
    format!("{}:47003", crate::get_conf("HOST_IP"))
}

pub fn connect_server() -> String {
    format!("http://{}:47003", crate::get_conf("HOST_IP"))
}
