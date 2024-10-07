/*
 * SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
 * SPDX-License-Identifier: Apache-2.0
 */

mod grpc_server;
mod method_bluechi;

use crate::grpc_server::StateManagerGrpcServer;
use common::statemanager::connection_server::ConnectionServer;
use tonic::transport::Server;

#[tokio::main]
async fn main() {
    let addr = common::statemanager::open_server()
        .parse()
        .expect("statemanager address parsing error");
    let state_manager_grpc_server = StateManagerGrpcServer::default();

    println!("Piccolod statemanager listening on {}", addr);

    let _ = Server::builder()
        .add_service(ConnectionServer::new(state_manager_grpc_server))
        .serve(addr)
        .await;
}

#[cfg(test)]
mod tests {
    /*#[tokio::test]
    async fn test_parsing() {
        let result =
            crate::grpc_server::make_action_for_scenario("scenario/version-display/action").await;
        println!("{:?}", result);
        assert!(result.is_ok());
    }*/

    #[test]
    fn test_ssh2() {
        use ssh2::Session;
        use std::net::TcpStream;

        let tcp = TcpStream::connect("10.157.19.234:22").unwrap();
        let mut session = Session::new().unwrap();
        session.set_tcp_stream(tcp);
        session.handshake().unwrap();
        session.userauth_password("sdv", "lge123").unwrap();
        assert!(session.authenticated());

        let mut channel = session.channel_session().unwrap();

        //channel.exec("sudo ln -s /root/piccolo_yaml/test.yaml /home/sdv/Music/2.yaml").unwrap();
        channel.exec("sudo rm -rf /home/sdv/Music/2.yaml").unwrap();
        //channel.send_eof().unwrap();
        channel.wait_eof().unwrap();
        channel.wait_close().unwrap();
    }
}
