/*
 * SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
 * SPDX-License-Identifier: Apache-2.0
 */

use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct Arguments {
    #[clap(subcommand)]
    /// command name.
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// make systemd service file
    Apply(File),
    /// delete systemd service file
    Delete(File),
}

#[derive(Args, Debug)]
pub struct File {
    /// file name
    pub name: String,
}
