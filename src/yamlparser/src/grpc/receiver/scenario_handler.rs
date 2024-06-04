/*
 * SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::file_handler;
use crate::grpc::sender::apiserver;
use crate::parser;
use common::yamlparser::connection_server::Connection;
use common::yamlparser::{SendRequest, SendResponse};
use tonic::{Code, Request, Response, Status};

#[derive(Default)]
pub struct YamlparserGrpcServer {}

#[tonic::async_trait]
impl Connection for YamlparserGrpcServer {
    async fn send(&self, request: Request<SendRequest>) -> Result<Response<SendResponse>, Status> {
        let req = request.into_inner();
        let command = req.request;

        match handle_msg(&command).await {
            Ok(s) => Ok(Response::new(SendResponse { response: s })),
            Err(e) => Err(Status::new(Code::Unavailable, e.to_string())),
        }
    }
}

async fn handle_msg(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let absolute_path = file_handler::get_absolute_file_path(path)?;
    let scenario = parser::scenario_parse(&absolute_path).await?;
    apiserver::send_msg_to_apiserver(scenario).await?;
    Ok("Success".to_string())
}
