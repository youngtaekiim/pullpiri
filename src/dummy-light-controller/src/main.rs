mod controller_manager;
mod dds;
mod grpc;

#[tokio::main]
async fn main() {
    let mut manager = controller_manager::controller_manager::new();
    manager.run().await;
}
