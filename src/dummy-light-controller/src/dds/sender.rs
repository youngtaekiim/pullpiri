use dust_dds::{
    dds_async::domain_participant_factory::DomainParticipantFactoryAsync,
    infrastructure::{qos::QosKind, status::NO_STATUS},
};
use dust_dds::dds_async::domain_participant::DomainParticipantAsync;
use dust_dds::dds_async::topic::TopicAsync;
use dust_dds::dds_async::publisher::PublisherAsync;
use dust_dds::dds_async::data_writer::DataWriterAsync;

#[allow(non_snake_case)]
pub mod lightState {
    #[derive(Debug, dust_dds::topic_definition::type_support::DdsType)]
    pub struct DataType {
        pub on: bool,
    }
}

#[allow(dead_code)]
pub struct DdsEventSender {
    participant: DomainParticipantAsync,
    topic: TopicAsync,
    publisher: PublisherAsync,
    writer: DataWriterAsync<lightState::DataType>,
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
                                .create_topic::<lightState::DataType>(
                                "LargeDataTopic",
                                "DataType",
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

            DdsEventSender { participant, topic, publisher, writer }
        }
    }
    pub async fn send(&self) {
        let msg = lightState::DataType{on: true,};
        self.writer.write(&msg, None).await.unwrap();
    }
}
