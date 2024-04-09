use crate::grpc::sender::statemanager;
use common::apiserver::request::request::RequestContent::{ControllerRequest, NodeRequest};
use common::apiserver::request::Request;
use common::apiserver::request_connection_server::RequestConnection;
use common::apiserver::Response;

#[derive(Default)]
pub struct GrpcRequestServer {}

#[tonic::async_trait]
impl RequestConnection for GrpcRequestServer {
    async fn send(
        &self,
        request: tonic::Request<Request>,
    ) -> Result<tonic::Response<Response>, tonic::Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let request = request.into_inner();
        let command = parse_to_server_command(&request);

        match statemanager::send_msg_to_statemanager(&command).await {
            Ok(v) => Ok(tonic::Response::new(Response {
                resp: v.into_inner().response,
            })),
            Err(e) => Err(tonic::Status::new(tonic::Code::Unavailable, e.to_string())),
        }
    }
}

fn parse_to_server_command(req: &Request) -> String {
    let mut ret = String::new();
    if let Some(request_command) = &req.request_content {
        match request_command {
            ControllerRequest(controller_request) => {
                ret = format!("{}", controller_request.controller_command().as_str_name());
            }
            NodeRequest(node_request) => {
                ret = format!(
                    "{}/{}",
                    node_request.node_command().as_str_name(),
                    &node_request.node_name
                );
            }
        }
    }
    ret
}
