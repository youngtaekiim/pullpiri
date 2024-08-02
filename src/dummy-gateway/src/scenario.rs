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

#[derive(Deserialize, Debug, Clone)]
pub struct ResourceScenario {
    pub name: String,
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
