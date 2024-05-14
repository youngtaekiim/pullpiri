use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Pod {
    apiVersion: String,
    kind: String,
    metadata: Metadata,
    spec: Spec,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    name: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Spec {
    #[serde(skip_serializing_if = "Option::is_none")]
    containers: Option<Vec<Container>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    initContainers: Option<Vec<Container>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    restartPolicy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    terminationGracePeriodSeconds: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hostIpc: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    runtimeClassName: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Container {
    image: Option<String>,
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ports: Option<Vec<Port>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    command: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    workingDir: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    env: Option<Vec<Env>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volumeMounts: Option<Vec<VolumeMount>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resources: Option<Resources>,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Port {
    #[serde(skip_serializing_if = "Option::is_none")]
    containerPort: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hostPort: Option<i32>,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Env {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct VolumeMount {
    #[serde(skip_serializing_if = "Option::is_none")]
    mountPath: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Resources {
    #[serde(skip_serializing_if = "Option::is_none")]
    requests: Option<Requests>,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Requests {
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory: Option<String>,
}
