/*
apiVersion: v1
kind: Volume
metadata:
  label: null
  name: vd-volume
spec:
  volumes:
    - name: x11
      mountPath: /tmp/.X11-unix
*/

use super::MetaData;
use super::super::workload::podspec;

#[allow(non_snake_case)]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Volume {
    apiVersion: String,
    kind: String,
    metadata: MetaData,
    spec: Option<VolumeSpec>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
struct VolumeSpec {
    spec: Option<Vec<podspec::Volume>>,
}
