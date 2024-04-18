pub mod method_controller;
pub mod method_node;
pub mod method_unit;

pub async fn send_dbus(msg: &str) -> Result<String, Box<dyn std::error::Error>> {
    println!("recv msg: {}\n", msg);
    let cmd: Vec<&str> = msg.split("/").collect();

    match cmd.len() {
        1 => method_controller::handle_cmd(cmd),
        2 => method_node::handle_cmd(cmd),
        3 => method_unit::handle_cmd(cmd),
        _ => Err("support only 1 ~ 3 parameters".into()),
    }
}