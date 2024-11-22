// SPDX-License-Identifier: Apache-2.0

use crate::DdsData;
use dust_dds::{
    dds_async::domain_participant_factory::DomainParticipantFactoryAsync,
    infrastructure::{qos::QosKind, status::NO_STATUS},
    subscription::sample_info::{ANY_INSTANCE_STATE, ANY_SAMPLE_STATE, ANY_VIEW_STATE},
};
use tokio::sync::mpsc::Sender;

#[derive(Debug, dust_dds::topic_definition::type_support::DdsType)]
pub struct ExteriorLightIntensity {
    pub value: f32,
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
        .create_topic::<ExteriorLightIntensity>(
            "ExteriorLightIntensity",
            "ExteriorLightIntensity",
            QosKind::Default,
            None,
            NO_STATUS,
        )
        .await
        .unwrap();
    let reader = subscriber
        .create_datareader::<ExteriorLightIntensity>(&topic, QosKind::Default, None, NO_STATUS)
        .await
        .unwrap();

    println!("make loop - ExteriorLightIntensity");
    loop {
        if let Ok(data_samples) = reader
            .take(10, ANY_SAMPLE_STATE, ANY_VIEW_STATE, ANY_INSTANCE_STATE)
            .await
        {
            let data: ExteriorLightIntensity = data_samples[0].data().unwrap();
            println!("Received:  charging state {}\n", data.value);

            let day_night = if data.value < 100.0 { "night" } else { "day" };

            let msg = DdsData {
                name: "ExteriorLightIntensity".to_string(),
                value: day_night.to_string(),
            };
            let _ = tx.send(msg).await;
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
