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
        let (tx_dds, rx_dds) = channel::<DdsData>(1);
        Manager {
            rx_rest,
            tx_dds,
            rx_dds,
        }
    }

    pub async fn run(&mut self) {
        tokio::spawn(launch_dds("gear", self.tx_dds.clone()));
        tokio::spawn(launch_dds("day", self.tx_dds.clone()));
        tokio::spawn(launch_dds("light", self.tx_dds.clone()));
        while let Some(scenario) = self.rx_rest.recv().await {
            // TODO parsing scenario
            println!("{:#?}", scenario);
            // TODO get condition and DDS criteria
            // assume that every process is ok
            self.launch_filter(&scenario).await;
        }
    }

    async fn launch_filter(&mut self, scenario: &ResourceScenario) {
        println!("launch filter {}\n", &scenario.name);

        let mut gear_status = false;
        let mut day_status = false;
        let mut gear_target_value = "";
        let mut day_target_value = "";

        let mut policy_state = false;

        let criteria = &scenario.condition.criteria;
        for criterion in criteria {
            if criterion.message.contains("Gear") {
                gear_target_value = &criterion.value;
                println!("gear target : {gear_target_value}");
            } else if criterion.message.contains("Day") {
                day_target_value = &criterion.value;
                println!("day target : {day_target_value}");
            }
        }

        while let Some(data) = self.rx_dds.recv().await {
            //println!("{:?}\n", data);

            if data.name == "gear" {
                gear_status = data.value == gear_target_value;
            } else if data.name == "day" {
                day_status = data.value == day_target_value;
            } else if data.name == "light" {
                //println!("Light : {}\n", data.value);
                if data.value == "OFF" && policy_state {
                    //let _ = crate::grpc::sender::lightcontroller::send(false).await;
                    println!("policy is applied and light is off. send TURN ON LIGHT msg\n");
                    let dds_sender = crate::grpc::sender::dds::DdsEventSender::new().await;
                    dds_sender.send().await;
                }
                continue;
            }

            println!("gear: {gear_status}, day: {day_status}\n");
            if gear_status && day_status && !policy_state {
                println!("meet conditions. apply policy");
                // send gRPC
                /*let value = scenario.policy.act.first().unwrap().value.clone();
                let ref_value = value.as_ref();
                let onoff = matches!(ref_value, "true" | "True");
                let _ = crate::grpc::sender::lightcontroller::send(onoff).await;*/
                policy_state = true;
            }
        }
        println!("terminate filter {}\n", &scenario.name);
    }
}

async fn launch_dds(name: &str, tx_dds: Sender<DdsData>) {
    let l = crate::listener::dds::DdsListener::new(name, tx_dds);
    l.run().await;
}
