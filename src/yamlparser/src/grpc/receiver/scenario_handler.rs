/*
 * SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::file_handler;
use crate::grpc::sender::apiserver;
use crate::parser;
use common::apiserver::scenario::Scenario;
use common::yamlparser::connection_server::Connection;
use common::yamlparser::{SendRequest, SendResponse};
use tonic::{Code, Request, Response, Status};

#[derive(Default)]
pub struct YamlparserGrpcServer {}

#[tonic::async_trait]
impl Connection for YamlparserGrpcServer {
    async fn send(&self, request: Request<SendRequest>) -> Result<Response<SendResponse>, Status> {
        let req = request.into_inner();
        let scenario = match handle_msg(&req.request) {
            Ok(scenario) => scenario,
            Err(e) => return Err(Status::new(Code::InvalidArgument, e.to_string())),
        };

        match apiserver::send_msg_to_apiserver(scenario).await {
            Ok(response) => Ok(tonic::Response::new(SendResponse {
                response: response.into_inner().resp,
            })),
            Err(e) => Err(Status::new(Code::Unavailable, e.to_string())),
        }
    }
}

pub fn handle_msg(path: &str) -> Result<Scenario, Box<dyn std::error::Error>> {
    let absolute_path = file_handler::get_absolute_file_path(path)?;
    let scenario = parser::scenario_parse(&absolute_path)?;
    Ok(scenario)
}
