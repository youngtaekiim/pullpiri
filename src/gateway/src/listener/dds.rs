use super::DdsData;
use dust_dds::{
    dds_async::domain_participant_factory::DomainParticipantFactoryAsync,
    infrastructure::{qos::QosKind, status::NO_STATUS},
    subscription::sample_info::{ANY_INSTANCE_STATE, ANY_SAMPLE_STATE, ANY_VIEW_STATE},
};
use tokio::sync::mpsc::Sender;

use lge::{batterycapacity::BatteryCapacity, chargingstatus::ChargingStatus};

// TOPIC NAME = /rt/piccolo/Gear_State
/*#[allow(non_snake_case)]
pub mod GearState {
    #[derive(Debug, dust_dds::topic_definition::type_support::DdsType)]
    pub struct DataType {
        pub state: String,
    }
}*/

// TOPIC NAME = /rt/piccolo/Day_Time
/*#[allow(non_snake_case)]
pub mod DayTime {
    #[derive(Debug, dust_dds::topic_definition::type_support::DdsType)]
    pub struct DataType {
        pub day: bool,
    }
}*/

/*#[allow(non_snake_case)]
pub mod LightState {
    #[derive(Debug, dust_dds::topic_definition::type_support::DdsType)]
    pub struct DataType {
        pub on: bool,
    }
}*/

pub struct DdsEventListener {
    name: String,
    tx: Sender<DdsData>,
}

impl DdsEventListener {
    pub fn new(name: &str, tx: Sender<DdsData>) -> Self {
        DdsEventListener {
            name: name.to_string(),
            tx,
        }
    }
}

impl Drop for DdsEventListener {
    fn drop(&mut self) {
        println!("drop DdsEventListener {}\n", self.name);
    }
}

impl super::EventListener for DdsEventListener {
    async fn run(&self) {
        let domain_id = 0;
        let participant_factory = DomainParticipantFactoryAsync::new();

        let participant = participant_factory
            .create_participant(domain_id, QosKind::Default, None, NO_STATUS)
            .await
            .unwrap();

        let subscriber = participant
            .create_subscriber(QosKind::Default, None, NO_STATUS)
            .await
            .unwrap();

        match self.name.as_str() {
            "/rt/piccolo/Battery_Capacity" => {
                let topic = participant
                    .create_topic::<BatteryCapacity::BatteryCapacityMsg>(
                        self.name.as_str(),
                        "BatteryCapacity::BatteryCapacityMsg",
                        QosKind::Default,
                        None,
                        NO_STATUS,
                    )
                    .await
                    .unwrap();
                let reader = subscriber
                    .create_datareader::<BatteryCapacity::BatteryCapacityMsg>(
                        &topic,
                        QosKind::Default,
                        None,
                        NO_STATUS,
                    )
                    .await
                    .unwrap();

                println!("make loop - capa");
                loop {
                    if let Ok(data_samples) = reader
                        .take(10, ANY_SAMPLE_STATE, ANY_VIEW_STATE, ANY_INSTANCE_STATE)
                        .await
                    {
                        let data: BatteryCapacity::BatteryCapacityMsg = data_samples[0].data().unwrap();
                        println!("Received:  battery capa {}\n", data.capacity);

                        let msg = DdsData {
                            name: self.name.clone(),
                            value: data.capacity,
                        };
                        let _ = self.tx.send(msg).await;
                    }
                    std::thread::sleep(std::time::Duration::from_millis(100));
                }
            }
            "/rt/piccolo/Charging_Status" => {
                let topic = participant
                    .create_topic::<ChargingStatus::ChargingStatusMsg>(
                        self.name.as_str(),
                        "ChargingStatus::ChargingStatusMsg",
                        QosKind::Default,
                        None,
                        NO_STATUS,
                    )
                    .await
                    .unwrap();
                let reader = subscriber
                    .create_datareader::<ChargingStatus::ChargingStatusMsg>(
                        &topic,
                        QosKind::Default,
                        None,
                        NO_STATUS,
                    )
                    .await
                    .unwrap();
                println!("make loop - charging");
                loop {
                    if let Ok(data_samples) = reader
                        .take(10, ANY_SAMPLE_STATE, ANY_VIEW_STATE, ANY_INSTANCE_STATE)
                        .await
                    {
                        let data: ChargingStatus::ChargingStatusMsg = data_samples[0].data().unwrap();
                        println!("Received:  charging state {}\n", data.state);

                        let msg = DdsData {
                            name: self.name.clone(),
                            value: data.state,
                        };
                        let _ = self.tx.send(msg).await;
                    }
                    std::thread::sleep(std::time::Duration::from_millis(100));
                }
            }
            _ => panic!("topic name is wrong"),
        };
    }
}
