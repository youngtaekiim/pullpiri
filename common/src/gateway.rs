/*
 * SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
 * SPDX-License-Identifier: Apache-2.0
 */

pub use api::proto::gateway::*;
pub const GATEWAY_CONNECT: &str = const_format::concatcp!("http://", crate::HOST_IP, ":47002");
