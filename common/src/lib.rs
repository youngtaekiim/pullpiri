pub mod apiserver;
pub mod etcd;
pub mod gateway;
pub mod statemanager;
pub mod yamlparser;
pub mod spec;

pub mod constants {
    pub use api::proto::constants::*;
}

pub const HOST_IP: &str = "10.159.57.33";
pub const YAML_STORAGE: &str = "/root/piccolo_yaml/";

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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
struct MetaData {
    name: String,
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
        self.podSpec.containers[0].image.clone()
    }

    pub fn get_operation(&self) -> String {
        self.operation.clone()
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
struct Operand {
    r#type: String,
    name: String,
    value: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct PodSpec {
    containers: Vec<Container>,
    volumes: Option<Vec<Volume>>,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Container {
    name: String,
    image: String,
    volumeMounts: Option<Vec<VolumeMount>>,
    env: Option<Vec<Env>>,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Volume {
    name: String,
    hostPath: HostPath,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct HostPath {
    path: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct VolumeMount {
    name: String,
    mountPath: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Env {
    name: String,
    value: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct KubePod {
    apiVersion: String,
    kind: String,
    metadata: MetaData,
    spec: PodSpec,
}

impl KubePod {
    pub fn new(name: &str, action: Action) -> KubePod {
        KubePod {
            apiVersion: "v1".to_string(),
            kind: "Pod".to_string(),
            metadata: MetaData {
                name: name.to_string(),
            },
            spec: action.podSpec,
        }
    }
}
