use common::dummylightcontroller::dummy_light_controller_server::DummyLightController;
use common::dummylightcontroller::PolicyLightOn;
use common::dummylightcontroller::Reply;
use tokio::sync::mpsc;
use crate::controller_manager;
use crate::controller_manager::message;
use crate::controller_manager::messageFrom;

pub struct GrpcServer {
    pub grpc_msg_tx: mpsc::Sender<message>,
}

#[tonic::async_trait]
impl DummyLightController for GrpcServer {
    async fn request_event(
        &self,
        request: tonic::Request<PolicyLightOn>,
    ) -> Result<tonic::Response<Reply>, tonic::Status> {
        println!("Got a request from {:?}", request.remote_addr());
        let req = request.into_inner();
        //println!("req msg : {:#?}", req);
        let msg = message {
            id: messageFrom::DUMMYGATEWAY,
            data: req.light_on,
        };
        let _ = self.grpc_msg_tx.send(msg).await;
        Ok(tonic::Response::new(Reply { is_ok: true }))
    }
}
