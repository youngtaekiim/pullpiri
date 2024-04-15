use crate::grpc::sender::statemanager;
use common::apiserver::update_workload_connection_server::UpdateWorkloadConnection;
use common::apiserver::updateworkload::UpdateWorkload;
use common::apiserver::Response;

#[derive(Default)]
pub struct GrpcUpdateServer {}

#[tonic::async_trait]
impl UpdateWorkloadConnection for GrpcUpdateServer {
    async fn send(
        &self,
        request: tonic::Request<UpdateWorkload>,
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

fn parse_to_server_command(req: &UpdateWorkload) -> String {
    format!(
        "{}/{}/{}",
        req.update_method().as_str_name(),
        req.node_name,
        req.unit_name
    )
}
