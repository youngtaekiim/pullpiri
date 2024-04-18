use crate::file_handler;
use crate::parser;
use common::yamlparser::connection_server::Connection;
use common::yamlparser::{SendRequest, SendResponse};
use tonic::{Code, Request, Response, Status};

#[derive(Default)]
pub struct YamlparserGrpcServer {}

#[tonic::async_trait]
impl Connection for YamlparserGrpcServer {
    async fn send(&self, request: Request<SendRequest>) -> Result<Response<SendResponse>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let req = request.into_inner();
        let command = req.request;
        println!("{}", command);

        match handle_msg(&command).await {
            Ok(s) => Ok(Response::new(SendResponse { response: s })),
            Err(e) => Err(Status::new(Code::Unavailable, e.to_string())),
        }
    }
}

async fn handle_msg(msg: &str) -> Result<String, Box<dyn std::error::Error>> {
    println!("recv msg: {}\n", msg);
    let file_path = file_handler::get_absolute_file_path(msg)?;
    parser::parser(&file_path).await?;
    Ok("Success".to_string())
}
