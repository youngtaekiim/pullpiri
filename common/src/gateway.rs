pub use api::proto::gateway::*;
pub const GATEWAY_CONNECT: &str = const_format::concatcp!("http://", crate::HOST_IP, ":47002");
