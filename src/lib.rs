use std::sync::mpsc;
use std::thread;

pub mod method;
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
    let cmd = Command::new(String::from("hello"));
    tx.send(cmd).unwrap();
}

fn handle_msgq(rx: mpsc::Receiver<Command>) {
    for received in rx {
        println!("{received}");
    }
    let result1 = method::list_nodes();
    println!("{:#?}", result1);
    //let result2 = method::list_node_units("nuc-cent");
    //println!("{:#?}", result2);
    let result3 = method::unit_lifecycle(method::Lifecycle::Restart, "nuc-cent", "pr-pingpong.service");
    println!("{:#?}", result3);
    //let result4 = method::enable_unit("nuc-cent", "bluechi-agent.service");
    //println!("{:#?}", result4);
    let result5 = method::disable_unit("nuc-cent", "bluechi-controller.service");
    println!("{:#?}", result5);
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
