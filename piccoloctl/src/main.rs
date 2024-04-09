mod cli_parser;
mod msg_sender;

use clap::Parser;
use common::apiserver::{get_controller_command, get_node_command, get_unit_command};
use common::apiserver::{ControllerCommand, NodeCommand, UpdateMethod};
use msg_sender::{send_request_msg, send_update_msg};

#[tokio::main]
async fn main() {
    let args = cli_parser::Arguments::parse();
    println!("{:?}", args);

    let res = match &args.command {
        cli_parser::Command::ListNode => {
            send_request_msg(get_controller_command(ControllerCommand::ListNode)).await
        }
        cli_parser::Command::DaemonReload => {
            send_request_msg(get_controller_command(ControllerCommand::DaemonReload)).await
        }
        cli_parser::Command::ListUnit(n) => {
            send_request_msg(get_node_command(NodeCommand::ListUnit, &n.node_name)).await
        }
        cli_parser::Command::Start(u) => {
            send_update_msg(get_unit_command(
                UpdateMethod::Start,
                &u.node_name,
                &u.unit_name,
            ))
            .await
        }
        cli_parser::Command::Stop(u) => {
            send_update_msg(get_unit_command(
                UpdateMethod::Stop,
                &u.node_name,
                &u.unit_name,
            ))
            .await
        }
        cli_parser::Command::Restart(u) => {
            send_update_msg(get_unit_command(
                UpdateMethod::Restart,
                &u.node_name,
                &u.unit_name,
            ))
            .await
        }
        cli_parser::Command::Reload(u) => {
            send_update_msg(get_unit_command(
                UpdateMethod::Reload,
                &u.node_name,
                &u.unit_name,
            ))
            .await
        }
        cli_parser::Command::Enable(u) => {
            send_update_msg(get_unit_command(
                UpdateMethod::Enable,
                &u.node_name,
                &u.unit_name,
            ))
            .await
        }
        cli_parser::Command::Disable(u) => {
            send_update_msg(get_unit_command(
                UpdateMethod::Disable,
                &u.node_name,
                &u.unit_name,
            ))
            .await
        }
    };

    match res {
        Ok(t) => println!("- SUCCESS -\n{}", t.into_inner().resp),
        Err(e) => println!("FAIL - {:#?}", e),
    }
}
