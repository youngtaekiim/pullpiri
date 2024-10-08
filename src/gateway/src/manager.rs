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
        tokio::spawn(launch_dds(
            "/rt/piccolo/Battery_Capacity",
            self.tx_dds.clone(),
        ));
        tokio::spawn(launch_dds(
            "/rt/piccolo/Charging_Status",
            self.tx_dds.clone(),
        ));

        let arc_rx_dds = Arc::clone(&self.rx_dds);
        let arc_filters = Arc::clone(&self.filters);
        tokio::spawn(handle_dds(arc_rx_dds, arc_filters));

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
}

async fn launch_dds(name: &str, tx_dds: Sender<DdsData>) {
    let l = crate::listener::dds::DdsEventListener::new(name, tx_dds);
    l.run().await;
}

async fn handle_dds(
    arc_rx_dds: Arc<Mutex<Receiver<DdsData>>>,
    arc_filters: Arc<Mutex<Vec<Filter>>>,
) {
    let mut rx_dds = arc_rx_dds.lock().await;
    while let Some(data) = rx_dds.recv().await {
        let mut filters = arc_filters.lock().await;
        if filters.is_empty() {
            continue;
        }

        let mut keep: Vec<bool> = Vec::new();
        for filter in filters.iter_mut() {
            let result = filter.check(data.clone()).await;
            keep.push(!result);
            if result {
                use crate::grpc::sender;
                let _ = sender::send(&filter.action_key).await;
            }
        }

        let mut iter = keep.iter();
        filters.retain(|_| *iter.next().unwrap());
    }
}
