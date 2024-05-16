/*
 * SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
 * SPDX-License-Identifier: Apache-2.0
 */

use clap::Parser;

/// This struct represents the arguments
#[derive(Parser, Debug)]
pub struct Arguments {
    /// This is the string argument we are expecting
    pub path: String,
}
