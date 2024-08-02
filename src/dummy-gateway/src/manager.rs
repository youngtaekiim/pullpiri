use tokio::sync::mpsc::{channel, Receiver, Sender};

use crate::listener::DdsData;
use crate::scenario::ResourceScenario;

#[derive(Clone)]
struct ScenarioStatus {
    name: String,
    listener: Vec<String>,
}

pub struct Manager {
    rx_rest: Receiver<ResourceScenario>,
    tx_dds: Sender<DdsData>,
    rx_dds: Receiver<DdsData>,
    scenarios: Vec<ScenarioStatus>,
}

impl Manager {
    pub fn new(rx_rest: Receiver<ResourceScenario>) -> Self {
        let (tx_dds, rx_dds) = channel::<DdsData>(50);
        Manager {
            rx_rest,
            tx_dds,
            rx_dds,
            scenarios: Vec::new(),
        }
    }

    pub async fn run(&mut self) {
        while let Some(scenario) = self.rx_rest.recv().await {
            // TODO parsing scenario
            println!("{:?}", scenario);
            // TODO get condition and DDS criteria
            // assume that every process is ok
            let ss = ScenarioStatus {
                name: "".to_string(),
                listener: vec!["gear".to_string(), "day".to_string()],
            };
            self.launch_filter(ss).await;
        }
    }

    async fn launch_filter(&mut self, ss: ScenarioStatus) {
        // TODO scenario condition parsing
        self.scenarios.push(ss);
        tokio::spawn(launch_dds("gear", self.tx_dds.clone()));
        tokio::spawn(launch_dds("day", self.tx_dds.clone()));

        while let Some(data) = self.rx_dds.recv().await {}
    }
}

async fn launch_dds(name: &str, tx_dds: Sender<DdsData>) {
    let l = crate::listener::dds::DdsListener::new(name, tx_dds);
    l.run().await;
}
