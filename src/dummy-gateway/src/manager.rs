use tokio::sync::mpsc::{channel, Receiver, Sender};

use crate::listener::DdsData;
use crate::scenario::ResourceScenario;

use std::collections::HashMap;

/*#[derive(Clone)]
struct ScenarioStatus {
    name: String,
    listener: Vec<crate::listener::dds::DdsListener>,
}*/

pub struct Manager {
    rx_rest: Receiver<ResourceScenario>,
    tx_dds: Sender<DdsData>,
    rx_dds: Receiver<DdsData>,
    scenarios: HashMap<String, Vec<crate::listener::dds::DdsListener>>,
}

impl Manager {
    pub fn new(rx_rest: Receiver<ResourceScenario>) -> Self {
        let (tx_dds, rx_dds) = channel::<DdsData>(50);
        Manager {
            rx_rest,
            tx_dds,
            rx_dds,
            scenarios: HashMap::new(),
        }
    }

    pub async fn run(&mut self) {
        while let Some(scenario) = self.rx_rest.recv().await {
            // TODO parsing scenario
            println!("{:?}", scenario);
            // TODO get condition and DDS criteria
            // assume that every process is ok

            self.launch_filter(&scenario.name).await;
        }
    }

    async fn launch_filter(&mut self, name: &str) {
        let dds_gear = crate::listener::dds::DdsListener::new("gear", self.tx_dds.clone());
        let dds_day = crate::listener::dds::DdsListener::new("day", self.tx_dds.clone());
        // TODO scenario condition parsing
        let vec = vec![dds_gear, dds_day];

        self.scenarios.insert(name.to_string(), vec);
        tokio::spawn(launch_dds("gear", self.tx_dds.clone()));
        tokio::spawn(launch_dds("day", self.tx_dds.clone()));

        let mut gear_status = false;
        let mut day_status = false;

        while let Some(data) = self.rx_dds.recv().await {
            // TODO : meet condition
            //let dds_name = data.name;
            //let dds_value = data.value;

            if data.name == "gear" && data.value == "driving" {
                gear_status = true;
            } else if data.name == "day" && data.value == "night" {
                day_status = true;
            }

            if gear_status && day_status {
                self.stop_scenario(name).await;
            }
        }
    }

    async fn stop_scenario(&mut self, name: &str) {
        for listener in self.scenarios.get_mut(name).unwrap() {
            listener.stop();
        }
    }
}

async fn launch_dds(name: &str, tx_dds: Sender<DdsData>) {
    let l = crate::listener::dds::DdsListener::new(name, tx_dds);
    l.run().await;
}
