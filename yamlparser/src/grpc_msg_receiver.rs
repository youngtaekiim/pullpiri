use crate::file_handler;
use crate::grpc_msg_sender;
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

async fn handle_msg(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    println!("recv msg: {}\n", path);
    let absolute_path = file_handler::get_absolute_file_path(path)?;
    let scenario = parser::parse(&absolute_path).await?;

    grpc_msg_sender::send_grpc_msg(scenario).await?;
    Ok("Success".to_string())
}
