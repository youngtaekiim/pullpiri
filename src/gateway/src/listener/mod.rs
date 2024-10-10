// SPDX-License-Identifier: Apache-2.0

pub mod dds;

pub trait EventListener {
    async fn run(&self);
}

#[derive(Debug, Clone)]
pub struct DdsData {
    pub name: String,
    pub value: String,
}
