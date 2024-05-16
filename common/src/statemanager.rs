/*
 * SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
 * SPDX-License-Identifier: Apache-2.0
 */

pub use api::proto::statemanager::*;
pub const STATE_MANAGER_OPEN: &str = const_format::concatcp!(crate::HOST_IP, ":47003");
pub const STATE_MANAGER_CONNECT: &str =
    const_format::concatcp!("http://", crate::HOST_IP, ":47003");
