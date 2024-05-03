mod file_handler;
mod grpc_msg_receiver;
mod grpc_msg_sender;
mod parser;

use common::yamlparser::connection_server::ConnectionServer;
use grpc_msg_receiver::YamlparserGrpcServer;
use tonic::transport::Server;

#[tokio::main]
async fn main() {
    let addr = common::yamlparser::YAML_PARSER_OPEN
        .parse()
        .expect("yamlparser address parsing error");
    let piccoloyaml_grpc_server = YamlparserGrpcServer::default();

    println!("yaml grpc server listening on {}", addr);
    let _ = Server::builder()
        .add_service(ConnectionServer::new(piccoloyaml_grpc_server))
        .serve(addr)
        .await;
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn parsing_update_scenario() {
        let path = std::path::PathBuf::from(
            "/root/work/projects-rust/piccolo-bluechi/doc/examples/scenario/update-scenario.yaml",
        );

        let result = crate::parser::parse(&path).await;
        println!("{:#?}", result);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn parsing_rollback_scenario() {
        let path = std::path::PathBuf::from(
            "/root/work/projects-rust/piccolo-bluechi/doc/examples/scenario/rollback-scenario.yaml",
        );

        let result = crate::parser::parse(&path).await;
        println!("{:#?}", result);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn parsing_omitted_scenario() {
        let path = std::path::PathBuf::from(
            "/root/work/projects-rust/piccolo-bluechi/doc/examples/scenario/omitted-scenario.yaml",
        );

        let result = crate::parser::parse(&path).await;
        println!("{:#?}", result);
        assert!(result.is_ok());
    }
}
