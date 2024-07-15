use super::super::workload::podspec;
use super::MetaData;

#[allow(non_snake_case)]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Volume {
    apiVersion: String,
    kind: String,
    metadata: MetaData,
    spec: Option<VolumeSpec>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct VolumeSpec {
    spec: Option<Vec<podspec::Volume>>,
}

impl Volume {
    pub fn get_volume(&self) -> VolumeSpec {
        self.spec.clone()
    }
}
