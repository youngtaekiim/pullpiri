use tokio::sync::mpsc::{channel, Receiver, Sender};

use crate::listener::DdsData;
use crate::scenario::ResourceScenario;

pub struct Manager {
    rx_rest: Receiver<ResourceScenario>,
    tx_dds: Sender<DdsData>,
    rx_dds: Receiver<DdsData>,
}

impl Manager {
    pub fn new(rx_rest: Receiver<ResourceScenario>) -> Self {
        let (tx_dds, rx_dds) = channel::<DdsData>(50);
        Manager {
            rx_rest,
            tx_dds,
            rx_dds,
        }
    }

    pub async fn run(&mut self) {
        tokio::spawn(launch_dds("gear", self.tx_dds.clone()));
        tokio::spawn(launch_dds("day", self.tx_dds.clone()));
        while let Some(scenario) = self.rx_rest.recv().await {
            // TODO parsing scenario
            println!("{:?}", scenario);
            // TODO get condition and DDS criteria
            // assume that every process is ok
            self.launch_filter(&scenario.name).await;
        }
    }

    async fn launch_filter(&mut self, name: &str) {
        println!("launch filter {}\n", name);

        let mut gear_status = false;
        let mut day_status = false;

        while let Some(data) = self.rx_dds.recv().await {
            println!("{:?}\n", data);
            if data.name == "gear" {
                gear_status = data.value == "drive";
            }
            if data.name == "day" {
                day_status = data.value == "night";
            }

            println!("gear: {gear_status}, day: {day_status}\n");
            if gear_status && day_status {
                println!("meet conditions. send policy");
                // send gRPC
                let _ = crate::grpc::sender::lightcontroller::send().await;
                break;
            }
        }
        println!("terminate filter {}\n", name);
    }
}

async fn launch_dds(name: &str, tx_dds: Sender<DdsData>) {
    let l = crate::listener::dds::DdsListener::new(name, tx_dds);
    l.run().await;
}
