/*
 * SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
 * SPDX-License-Identifier: Apache-2.0
 */

use super::workload::podspec::PodSpec;
use super::MetaData;

#[allow(non_snake_case)]
#[derive(Debug, serde::Deserialize, PartialEq)]
pub struct Scenario {
    apiVersion: String,
    kind: String,
    metadata: MetaData,
    spec: ScenarioSpec,
}

impl Scenario {
    pub fn get_name(&self) -> String {
        self.metadata.name.clone()
    }

    pub fn get_conditions(&self) -> Option<Condition> {
        self.spec.conditions.clone()
    }

    pub fn get_actions(&self) -> Vec<Action> {
        self.spec.actions.clone()
    }
}

#[derive(Debug, serde::Deserialize, PartialEq)]
struct ScenarioSpec {
    conditions: Option<Condition>,
    actions: Vec<Action>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Condition {
    express: String,
    value: String,
    operands: Operand,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Action {
    operation: String,
    podSpec: PodSpec,
}

impl Action {
    pub fn get_image(&self) -> String {
        self.podSpec.get_image()
    }

    pub fn get_operation(&self) -> String {
        self.operation.clone()
    }

    pub fn get_podspec(&self) -> PodSpec {
        self.podSpec.clone()
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
struct Operand {
    r#type: String,
    name: String,
    value: String,
}
