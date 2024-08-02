mod dds;
mod grpc;
mod controller_manager;

#[tokio::main]
async fn main() {
    let mut manager = controller_manager::controller_manager::new();
    manager.run().await;
}
