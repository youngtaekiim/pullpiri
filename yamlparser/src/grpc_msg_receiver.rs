use crate::file_handler;
use crate::parser;
use common::yamlparser::connection_server::Connection;
use common::yamlparser::{SendRequest, SendResponse};

#[derive(Default)]
pub struct YamlparserGrpcServer {}

#[tonic::async_trait]
impl Connection for YamlparserGrpcServer {
    async fn send(
        &self,
        request: tonic::Request<SendRequest>,
    ) -> Result<tonic::Response<SendResponse>, tonic::Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let req = request.into_inner();
        let command = req.request;
        println!("{}", command);

        match handle_msg(&command).await {
            Ok(v) => Ok(tonic::Response::new(SendResponse { response: v })),
            Err(e) => Err(tonic::Status::new(tonic::Code::Unavailable, e.to_string())),
        }
    }
}

async fn handle_msg(msg: &str) -> Result<String, Box<dyn std::error::Error>> {
    println!("recv msg: {}\n", msg);
    let file_path = file_handler::get_absolute_file_path(msg)?;
    parser::parser(&file_path).await?;
    Ok("Success".to_string())
}
