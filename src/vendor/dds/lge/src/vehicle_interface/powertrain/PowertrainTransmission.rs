// SPDX-License-Identifier: Apache-2.0

use crate::DdsData;
use dust_dds::{
    dds_async::domain_participant_factory::DomainParticipantFactoryAsync,
    infrastructure::{qos::QosKind, status::NO_STATUS},
    subscription::sample_info::{ANY_INSTANCE_STATE, ANY_SAMPLE_STATE, ANY_VIEW_STATE},
};
use tokio::sync::mpsc::Sender;

#[derive(Debug, dust_dds::topic_definition::type_support::DdsType)]
pub struct PowertrainTransmissionCurrentGear {
    pub value: i32,
}

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
        .create_topic::<PowertrainTransmissionCurrentGear>(
            "PowertrainTransmissionCurrentGear",
            "PowertrainTransmissionCurrentGear",
            QosKind::Default,
            None,
            NO_STATUS,
        )
        .await
        .unwrap();
    let reader = subscriber
        .create_datareader::<PowertrainTransmissionCurrentGear>(
            &topic,
            QosKind::Default,
            None,
            NO_STATUS,
        )
        .await
        .unwrap();

    println!("make loop - PowertrainTransmissionCurrentGear");
    loop {
        if let Ok(data_samples) = reader
            .take(10, ANY_SAMPLE_STATE, ANY_VIEW_STATE, ANY_INSTANCE_STATE)
            .await
        {
            let data: PowertrainTransmissionCurrentGear = data_samples[0].data().unwrap();
            println!("Received:  charging state {}\n", data.value);

            let gear = match data.value {
                //// value: 0=Neutral, 1/2/..=Forward, -1/-2/..=Reverse, 126: Parking, 127: Drive
                0 => "neutral",
                -1 => "reverse",
                126 => "parking",
                127 => "driving",
                _ => "unknown",
            };

            let msg = DdsData {
                name: "PowertrainTransmissionCurrentGear".to_string(),
                value: gear.to_string(),
            };
            let _ = tx.send(msg).await;
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
