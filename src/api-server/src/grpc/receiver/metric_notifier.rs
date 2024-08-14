/*
 * SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
 * SPDX-License-Identifier: Apache-2.0
 */

use common::apiserver::metric_notifier::{ContainerList, ImageList, PodList, Response};
use common::apiserver::metric_notifier_server::MetricNotifier;
use tonic::Request;

type GrpcResult = Result<tonic::Response<Response>, tonic::Status>;

#[derive(Default)]
pub struct GrpcMetricServer {}

#[tonic::async_trait]
impl MetricNotifier for GrpcMetricServer {
    async fn send_image_list(&self, request: Request<ImageList>) -> GrpcResult {
        println!("Got a request from {:?}", request.remote_addr());

        let image_list = request.into_inner();
        println!("image\n{:?}", image_list);

        Ok(tonic::Response::new(Response { success: true }))
    }

    async fn send_container_list(&self, request: Request<ContainerList>) -> GrpcResult {
        println!("Got a request from {:?}", request.remote_addr());

        let container_list = request.into_inner();
        println!("container\n{:?}", container_list);

        Ok(tonic::Response::new(Response { success: true }))
    }

    async fn send_pod_list(&self, request: Request<PodList>) -> GrpcResult {
        println!("Got a request from {:?}", request.remote_addr());

        let pod_list = request.into_inner();
        println!("pod\n{:?}", pod_list);

        Ok(tonic::Response::new(Response { success: true }))
    }
}
