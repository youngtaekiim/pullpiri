// SPDX-License-Identifier: Apache-2.0

use tokio::sync::mpsc::Sender;
use super::DdsData;

// TOPIC NAME = /rt/piccolo/Battery_Capacity
pub mod BatteryCapacity {
    #[derive(Debug, dust_dds::topic_definition::type_support::DdsType)]
    pub struct BatteryCapacityMsg {
        pub capacity: String,
    }
}

use dust_dds::{
    dds_async::domain_participant_factory::DomainParticipantFactoryAsync,
    infrastructure::{qos::QosKind, status::NO_STATUS},
    subscription::sample_info::{ANY_INSTANCE_STATE, ANY_SAMPLE_STATE, ANY_VIEW_STATE},
};

pub async fn run(tx: Sender<DdsData>) {
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

    let topic = participant
        .create_topic::<BatteryCapacity::BatteryCapacityMsg>(
            "/rt/piccolo/Battery_Capacity",
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
            let data: BatteryCapacity::BatteryCapacityMsg =
                data_samples[0].data().unwrap();
            println!("Received:  battery capa {}\n", data.capacity);

            let msg = DdsData {
                name: "/rt/piccolo/Battery_Capacity".to_string(),
                value: data.capacity,
            };
            let _ = tx.send(msg).await;
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

