use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct ResourceScenario {
    name: String,
    condition: Condition,
    policy: Policy,
}

#[derive(Deserialize, Debug, Clone)]
struct Condition {
    name: Option<String>,
    criteria: Vec<Criterion>,
}

#[derive(Deserialize, Debug, Clone)]
struct Policy {
    name: Option<String>,
    act: Vec<Act>,
}

#[derive(Deserialize, Debug, Clone)]
struct Criterion {
    message: String,
    value: String,
    operand: String,
}

#[derive(Deserialize, Debug, Clone)]
struct Act {
    message: String,
    value: String,
}