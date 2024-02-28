use rustdds::*;
use serde::{Deserialize, Serialize};
use std::sync::mpsc;
use std::thread;

mod method_controller;
mod method_node;
mod method_unit;

struct Command {
    cmd_name: String,
}
impl Command {
    fn new(cmd_name: String) -> Self {
        Self { cmd_name }
    }
}
impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "str : {}", self.cmd_name)
    }
}

fn ddsmsg_to_msgq(tx: mpsc::Sender<Command>) {
    let domain_participant = DomainParticipant::new(0).unwrap();
    let qos = QosPolicyBuilder::new()
        .reliability(policy::Reliability::Reliable {
            max_blocking_time: rustdds::Duration::ZERO,
        })
        .build();
    let subscriber = domain_participant.create_subscriber(&qos).unwrap();
    let piccolo_internal_topic = domain_participant
        .create_topic(
            "piccolo_internal_topic".to_string(),
            "PiccoloInternalDdsType".to_string(),
            &qos,
            TopicKind::NoKey,
        )
        .unwrap();
    #[derive(Serialize, Deserialize, Debug)]
    struct PiccoloInternalDdsType {
        msg: String,
    }
    let mut reader: no_key::DataReader<PiccoloInternalDdsType> = subscriber
        .create_datareader_no_key::<PiccoloInternalDdsType, CDRDeserializerAdapter<PiccoloInternalDdsType>>(&piccolo_internal_topic, None)
        .unwrap();
    loop {
        let msg_struct = if let Ok(Some(value)) = reader.take_next_sample() {
            value
        } else {
            // no data has arrived
            continue;
        };
        let received_msg = &msg_struct.value().msg;
        let cmd = Command::new(String::from(received_msg));
        tx.send(cmd).unwrap();
        thread::sleep(std::time::Duration::from_millis(500));
    }
}

fn handle_msgq(rx: mpsc::Receiver<Command>) {
    for received in rx {
        println!("{}\n", received);

        let cmd: Vec<&str> = received.cmd_name.split("/").collect();
        let result = match cmd.len() {
            1 => method_controller::handle_cmd(cmd),
            2 => method_node::handle_cmd(cmd),
            3 => method_unit::handle_cmd(cmd),
            _ => Err("support only 1 ~ 3 parameters".into()),
        };

        match result {
            Ok(v) => println!("{}", v),
            Err(e) => println!("Error : {}", e.to_string()),
        }
    }
}

pub fn run() {
    let (tx, rx) = mpsc::channel();

    let mpsc_receiver = thread::spawn(move || {
        handle_msgq(rx);
    });

    let mpsc_sender = thread::spawn(move || {
        ddsmsg_to_msgq(tx);
    });

    mpsc_receiver.join().unwrap();
    mpsc_sender.join().unwrap();
}
