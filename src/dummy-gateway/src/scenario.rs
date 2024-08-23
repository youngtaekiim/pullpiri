use serde::Deserialize;

/*
Scenario example

JSON *****************
{
  "name": "fakename",
  "condition": {
    "name": null,       // Optional
    "criteria": [
      {
        "message": "rt/piccolo/gear_state",
        "value": "driving",
        "operand": "equal"
      },
      {
        "message": "rt/piccolo/day",
        "value": "night",
        "operand": "equal"
      }
    ]
  },
  "policy": {
    "name": null,       // Optional
    "act": [
      {
        "message": "rt/piccolo/light_on",
        "value": "true"
      }
    ]
  }
}
**********************

YAML *****************
name: fakename
condition:
  name: null        # Optional
  criteria:
  - message: rt/piccolo/gear_state
    value: driving
    operand: equal
  - message: rt/piccolo/day
    value: night
    operand: equal
policy:
  name: null        # Optional
  act:
  - message: rt/piccolo/light_on
    value: 'true'
**********************
*/

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct ResourceScenario {
    pub name: String,
    pub condition: Condition,
    pub policy: Policy,
    pub route: Option<bool>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct Condition {
    name: Option<String>,
    pub criteria: Vec<Criterion>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct Policy {
    name: Option<String>,
    pub act: Vec<Act>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct Criterion {
    pub message: String,
    pub value: String,
    pub operand: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct Act {
    pub message: String,
    pub value: String,
}
