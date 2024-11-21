// SPDX-License-Identifier: Apache-2.0

// TOPIC NAME = /rt/piccolo/Charging_Status
pub mod ChargingStatus {
    #[derive(Debug, dust_dds::topic_definition::type_support::DdsType)]
    pub struct ChargingStatusMsg {
        pub state: String,
    }
}


// SPDX-License-Identifier: Apache-2.0

use tokio::sync::mpsc::Sender;
use super::DdsData;

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
        .create_topic::<ChargingStatus::ChargingStatusMsg>(
            "/rt/piccolo/Charging_Status",
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

    println!("make loop - capa");
    loop {
        if let Ok(data_samples) = reader
            .take(10, ANY_SAMPLE_STATE, ANY_VIEW_STATE, ANY_INSTANCE_STATE)
            .await
        {
            let data: ChargingStatus::ChargingStatusMsg =
                data_samples[0].data().unwrap();
            println!("Received:  charging state {}\n", data.state);

            let msg = DdsData {
                name: "/rt/piccolo/Charging_Status".to_string(),
                value: data.state,
            };
            let _ = tx.send(msg).await;
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

