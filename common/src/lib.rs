pub mod apiserver;
pub mod etcd;
pub mod gateway;
pub mod statemanager;
pub mod yamlparser;

pub mod constants {
    pub use api::proto::constants::*;
}

pub const HOST_IP: &str = "10.159.57.33";
