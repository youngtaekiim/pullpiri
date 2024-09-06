mod filter;
mod grpc;
mod listener;
mod manager;

use manager::GatewayManager;
/*use tokio::sync::mpsc::{channel, Receiver, Sender};

async fn running_manager() {

}

async fn running_grpc() {

}*/

#[tokio::main]
async fn main() {
    //let (tx_grpc, rx_grpc) = channel(50);
    let mut gateway_manager = GatewayManager::new();
    gateway_manager.run().await;
}
