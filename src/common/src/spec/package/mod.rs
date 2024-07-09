pub mod model;
pub mod volume;
pub mod network;

use super::MetaData;

#[allow(non_snake_case)]
#[derive(Debug, serde::Deserialize, PartialEq)]
pub struct Package {
    apiVersion: String,
    kind: String,
    metadata: MetaData,
    spec: PackageSpec,
    status: PackageStatus,
}

#[allow(non_snake_case)]
#[derive(Debug, serde::Deserialize, PartialEq)]
pub struct PackageSpec {
    pattern: Vec<Pattern>,
    models: Vec<Model>,
}

#[derive(Debug, serde::Deserialize, PartialEq)]
struct Pattern {
    r#type: String,
}

#[derive(Debug, serde::Deserialize, PartialEq)]
struct Model {
    name: String,
    resources: Resource,
}

#[derive(Debug, serde::Deserialize, PartialEq)]
struct Resource {
    volume: String,
    network: String,
}

#[derive(Debug, serde::Deserialize, PartialEq)]
pub struct PackageStatus {
    model: Vec<ModelStatus>,
}

#[derive(Debug, serde::Deserialize, PartialEq)]
pub struct ModelStatus {
    name: String,
    state: ModelStatusState,
}

#[derive(Debug, serde::Deserialize, PartialEq)]
enum ModelStatusState {
    None,
    Running,
    Error,
}