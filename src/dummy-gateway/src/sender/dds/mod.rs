use dust_dds::dds_async::data_writer::DataWriterAsync;
use dust_dds::dds_async::domain_participant::DomainParticipantAsync;
use dust_dds::dds_async::publisher::PublisherAsync;
use dust_dds::dds_async::topic::TopicAsync;
use dust_dds::{
    dds_async::domain_participant_factory::DomainParticipantFactoryAsync,
    infrastructure::{qos::QosKind, status::NO_STATUS},
};

#[allow(non_snake_case)]
pub mod TurnLight {
    #[derive(Debug, dust_dds::topic_definition::type_support::DdsType)]
    pub struct DataType {
        pub operation: String,
    }
}

#[allow(dead_code)]
pub struct DdsEventSender {
    participant: DomainParticipantAsync,
    topic: TopicAsync,
    publisher: PublisherAsync,
    writer: DataWriterAsync<TurnLight::DataType>,
}

impl DdsEventSender {
    pub async fn new() -> Self {
        {
            let domain_id = 0;

            let participant_factory = DomainParticipantFactoryAsync::new();
            let participant = participant_factory
                .create_participant(domain_id, QosKind::Default, None, NO_STATUS)
                .await
                .unwrap();
            let topic = participant
                .create_topic::<TurnLight::DataType>(
                    "/rt/piccolo/Turn_Light",
                    "TurnLight::DataType",
                    QosKind::Default,
                    None,
                    NO_STATUS,
                )
                .await
                .unwrap();

            let publisher = participant
                .create_publisher(QosKind::Default, None, NO_STATUS)
                .await
                .unwrap();

            let writer = publisher
                .create_datawriter(&topic, QosKind::Default, None, NO_STATUS)
                .await
                .unwrap();

            DdsEventSender {
                participant,
                topic,
                publisher,
                writer,
            }
        }
    }

    pub async fn send(&self, onoff: &str) {
        let msg = TurnLight::DataType {
            operation: onoff.to_string(),
        };
        self.writer.write(&msg, None).await.unwrap();
    }
}
