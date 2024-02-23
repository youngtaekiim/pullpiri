use rustdds::*;
use serde::{Deserialize, Serialize};
use std::thread;

fn main() {
    #[derive(Serialize, Deserialize, Debug)]
    struct PiccoloInternalDdsType {
        msg: String,
    }

    let domain_participant = DomainParticipant::new(0).unwrap();
    let qos = QosPolicyBuilder::new()
        .reliability(policy::Reliability::Reliable {
            max_blocking_time: rustdds::Duration::ZERO,
        })
        .build();
    let publisher = domain_participant.create_publisher(&qos).unwrap();
    let piccolo_internal_topic = domain_participant
        .create_topic(
            "let piccolo_internal_topic".to_string(),
            "PiccoloInternalDdsType".to_string(),
            &qos,
            TopicKind::NoKey,
        )
        .unwrap();

    let writer: no_key::DataWriter<PiccoloInternalDdsType> = publisher
        .create_datawriter_no_key::<PiccoloInternalDdsType, CDRSerializerAdapter<PiccoloInternalDdsType>>(&piccolo_internal_topic, None)
        .unwrap();

    let dds_sender = thread::spawn(move || loop {
        let some_data = PiccoloInternalDdsType {
            msg: String::from("hello123"),
        };
        let result = writer.write(some_data, None);
        let _result = match result {
            Ok(t) => t,
            Err(error) => {
                panic!("error : {:?}", error)
            }
        };
        thread::sleep(std::time::Duration::from_millis(1000));
    });
    dds_sender.join().unwrap();
}
