use crate::filter::Filter;
use crate::listener::EventListener;
use common::gateway::Condition;
use tokio::sync::mpsc::{Sender, Receiver, channel};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Manager {
    rx_grpc: Receiver<Condition>,
    filters: Arc<Mutex<Vec<Filter>>>,
}

impl Manager {
    pub fn new(rx: Receiver<Condition>) -> Self {
        Manager { rx_grpc: rx , filters: Arc::new(Mutex::new(Vec::new())) }
    }

    pub async fn run(&mut self) {

        while let Some(req) = self.rx_grpc.recv().await {
            let name = req.name.clone();
            if event::get(&name).is_some() {
                println!("Already exist scenario.\n");
                continue;
            }
            let event: Event = parser::parse(&name, req.target).await;

            println!("{:#?}\n", req);
            println!("{:#?}\n", event);

            event::insert(&name, event);
            tokio::spawn(launch_dds(name));
        }
    }
}

async fn launch_dds(name: String) {
    let l = crate::listener::dds::DdsEventListener::new(name);
    l.run().await;
}
