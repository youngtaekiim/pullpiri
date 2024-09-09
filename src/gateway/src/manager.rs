use crate::filter::Filter;
use crate::listener::{DdsData, EventListener};
use common::gateway::Condition;
use std::sync::Arc;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::sync::Mutex;

pub struct Manager {
    rx_grpc: Arc<Mutex<Receiver<Condition>>>,
    tx_dds: Sender<DdsData>,
    rx_dds: Arc<Mutex<Receiver<DdsData>>>,
    filters: Arc<Mutex<Vec<Filter>>>,
}

impl Manager {
    pub fn new(rx: Receiver<Condition>) -> Self {
        let (tx_dds, rx_dds) = channel::<DdsData>(10);
        Manager {
            rx_grpc: Arc::new(Mutex::new(rx)),
            tx_dds,
            rx_dds: Arc::new(Mutex::new(rx_dds)),
            filters: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn run(&mut self) {
        tokio::spawn(launch_dds("gear", self.tx_dds.clone()));
        tokio::spawn(launch_dds("day", self.tx_dds.clone()));

        tokio::spawn(self.handle_dds());

        let arc_rx_grpc = Arc::clone(&self.rx_grpc);
        let mut rx_grpc = arc_rx_grpc.lock().await;
        while let Some(condition) = rx_grpc.recv().await {
            self.launch_filter(&condition).await;
            // TODO : need remove function
        }
    }

    async fn launch_filter(&mut self, condition: &Condition) {
        println!("launch filter {}\n", &condition.name);

        let f = Filter::new(&condition.name).await;
        let arc_filters = Arc::clone(&self.filters);
        let mut filters = arc_filters.lock().await;
        filters.push(f);
    }

    async fn handle_dds(&mut self) {
        let arc_rx_dds = Arc::clone(&self.rx_dds);
        let arc_filters = Arc::clone(&self.filters);

        let mut rx_dds = arc_rx_dds.lock().await;
        while let Some(data) = rx_dds.recv().await {
            let mut filters = arc_filters.lock().await;
            if filters.is_empty() {
                continue;
            }

            for filter in filters.iter_mut() {
                match data.name.as_str() {
                    "gear" => filter.set_status(0, &data.value).await,
                    "day" => filter.set_status(1, &data.value).await,
                    _ => continue,
                }
            }
        }
    }
}

async fn launch_dds(name: &str, tx_dds: Sender<DdsData>) {
    let l = crate::listener::dds::DdsEventListener::new(name, tx_dds);
    l.run().await;
}
