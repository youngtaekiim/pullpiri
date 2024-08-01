/*
* SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
* SPDX-License-Identifier: Apache-2.0
*/

use scenario::ResourceScenario;
use tokio::sync::mpsc::{UnboundedSender, unbounded_channel};

mod route;
mod manager;
mod scenario;

async fn running_manager() {
    let mut manager = manager::Manager::new();
    manager.run().await;
}

async fn running_rest(tx: UnboundedSender<ResourceScenario>) {
    let app = axum::Router::new()
        .route("/scenario", axum::routing::post(route::import_scenario));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:47098")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app.with_state(tx).into_make_service()).await.unwrap();
}

#[tokio::main]
async fn main() {
    let (tx_rest, rx_rest) = unbounded_channel::<ResourceScenario>();
    let f_manage = running_manager();
    let f_rest = running_rest(tx_rest);

    tokio::join!(f_manage, f_rest);
}
