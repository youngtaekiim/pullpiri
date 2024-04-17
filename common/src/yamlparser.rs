pub use api::proto::yamlparser::*;
pub const YAML_PARSER_OPEN: &str = const_format::concatcp!(crate::HOST_IP, ":47004");
pub const YAML_PARSER_CONNECT: &str = const_format::concatcp!("http://", crate::HOST_IP, ":47004");
