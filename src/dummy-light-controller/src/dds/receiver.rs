use crate::controller_manager::message;
use crate::controller_manager::messageFrom;
use dust_dds::{
    dds_async::domain_participant_factory::DomainParticipantFactoryAsync,
    infrastructure::{qos::QosKind, status::NO_STATUS},
    subscription::sample_info::{ANY_INSTANCE_STATE, ANY_SAMPLE_STATE, ANY_VIEW_STATE},
};
use tokio::sync::mpsc;

#[allow(non_snake_case)]
pub mod lightState {
    #[derive(Debug, dust_dds::topic_definition::type_support::DdsType)]
    pub struct DataType {
        pub on: bool,
    }
}

pub struct DdsEventListener {
    grpc_tx: mpsc::Sender<message>,
}

impl DdsEventListener {
    pub fn new(grpc_tx: mpsc::Sender<message>) -> Self {
        DdsEventListener { grpc_tx }
    }
}

impl Drop for DdsEventListener {
    fn drop(&mut self) {
        let topic_name = "/rt/piccolo/Light_State";
        println!("drop DdsEventListener {}\n", topic_name);
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

        let topic = participant
            .create_topic::<lightState::DataType>(
                "/rt/piccolo/Light_State",
                "lightState::DataType",
                QosKind::Default,
                None,
                NO_STATUS,
            )
            .await
            .unwrap();

        let subscriber = participant
            .create_subscriber(QosKind::Default, None, NO_STATUS)
            .await
            .unwrap();

        let reader = subscriber
            .create_datareader::<lightState::DataType>(&topic, QosKind::Default, None, NO_STATUS)
            .await
            .unwrap();

        loop {
            if let Ok(data_samples) = reader
                .take(10, ANY_SAMPLE_STATE, ANY_VIEW_STATE, ANY_INSTANCE_STATE)
                .await
            {
                let receive_data = data_samples[0].data().unwrap();

                println!("dds Received: {:?}\n", receive_data);
                let msg = message {
                    id: messageFrom::LightSource,
                    data: receive_data.on,
                };
                let _ = self.grpc_tx.send(msg).await;
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }
}
