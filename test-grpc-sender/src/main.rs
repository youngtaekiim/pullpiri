use common::yamlparser::connection_client::ConnectionClient;
use common::yamlparser::SendRequest;
use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let send: SendRequest = SendRequest {
        request: path.to_string(),
    };

    let mut client = ConnectionClient::connect(common::yamlparser::YAML_PARSER_CONNECT)
        .await
        .unwrap_or_else(|err| {
            println!("FAIL - {}\ncannot connect to gRPC server", err);
            std::process::exit(1);
        });

    match client.send(tonic::Request::new(send)).await {
        Ok(v) => println!("\nSUCCESS\n{:?}\n", v),
        Err(e) => println!("\nFAIL\n{:?}\n", e),
    }
}
