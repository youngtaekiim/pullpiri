/*
 * SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
 * SPDX-License-Identifier: Apache-2.0
 */

use common::yamlparser::connection_client::ConnectionClient;
use common::yamlparser::SendRequest;
use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let send: SendRequest = SendRequest {
        request: path.to_string(),
    };

    //let mut client = ConnectionClient::connect(common::yamlparser::connect_server())
    let mut client = ConnectionClient::connect("http://10.157.19.218:47004")
        .await
        .expect("- FAIL - \ncannot connect to yamlparser server");

    match client.send(tonic::Request::new(send)).await {
        Ok(v) => println!("\nSUCCESS\n{:?}\n", v),
        Err(e) => println!("\nFAIL\n{:#?}\n", e),
    }
}
