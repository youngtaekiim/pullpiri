use crate::dds::EventListener;
use common::dummylightcontroller::dummy_light_controller_server::DummyLightControllerServer;
use tokio::sync::mpsc;
use tonic::transport::Server;

#[allow(non_camel_case_types)]
pub enum messageFrom {
    LightSource,
    DummyGateway,
}

#[allow(non_camel_case_types)]
pub struct message {
    pub id: messageFrom,
    pub data: bool,
}

#[allow(non_camel_case_types)]
pub struct controller_manager {
    grpc_rx: mpsc::Receiver<message>,
    grpc_tx: mpsc::Sender<message>,
    light_status: bool,
    policy_status: bool,
}

impl controller_manager {
    pub fn new() -> Self {
        let (grpc_tx, grpc_rx) = mpsc::channel(100);
        controller_manager {
            grpc_rx,
            grpc_tx,
            light_status: false,
            policy_status: false,
        }
    }

    pub fn executer(&self) {
        if self.policy_status && !self.light_status {
            //To-Do somerhing
        }
    }

    pub async fn run(&mut self) {
        tokio::spawn(launch_dds(self.grpc_tx.clone()));
        tokio::spawn(grpc_server(self.grpc_tx.clone()));

        while let Some(req) = self.grpc_rx.recv().await {
            let id = req.id;
            let data = req.data;
            match id {
                messageFrom::LightSource => {
                    self.light_status = data;
                    self.executer();
                }
                messageFrom::DummyGateway => self.policy_status = data,
            }
            println!("policy activate is {}", self.policy_status);
            println!("light state is {}", self.light_status);
        }
    }
}
async fn grpc_server(tx: mpsc::Sender<message>) {
    let server = crate::grpc::receiver::GrpcServer {
        grpc_msg_tx: tx.clone(),
    };
    let addr = common::dummylightcontroller::open_server()
        .parse()
        .expect("dummy-light-controller address parsing error");

    println!("Piccolod dummy-light-controller listening on {}", addr);

    let _ = Server::builder()
        .add_service(DummyLightControllerServer::new(server))
        .serve(addr)
        .await;
}

async fn launch_dds(grpc_tx: mpsc::Sender<message>) {
    let l = crate::dds::receiver::DdsEventListener::new(grpc_tx);
    l.run().await;
}
