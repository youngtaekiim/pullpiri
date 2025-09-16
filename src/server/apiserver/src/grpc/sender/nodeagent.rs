use common::nodeagent::{
    node_agent_connection_client::NodeAgentConnectionClient,
    HandleYamlRequest, HandleYamlResponse,
};
use tonic::{Request, Response, Status};

// Send to a specific node using its IP address
pub async fn send_to_node(action: HandleYamlRequest, node_ip: String) -> Result<Response<HandleYamlResponse>, Status> {
    let addr = format!("http://{}:47004", node_ip);
    
    let client_result = NodeAgentConnectionClient::connect(addr).await;
    
    match client_result {
        Ok(mut client) => client.handle_yaml(Request::new(action)).await,
        Err(e) => {
            eprintln!("Error connecting to NodeAgent at {}: {}", node_ip, e);
            Err(Status::unavailable(format!("Failed to connect to NodeAgent at {}: {}", node_ip, e)))
        }
    }
}

pub async fn send(action: HandleYamlRequest) -> Result<Response<HandleYamlResponse>, Status> {
    // Connect to NodeAgent service using the settings file
    let config = common::setting::get_config();
    let node_ip = config.host.ip.clone();
    
    send_to_node(action, node_ip).await
}

pub async fn send_guest(action: HandleYamlRequest) -> Result<Response<HandleYamlResponse>, Status> {
    // Connect to guest NodeAgent service
    let guest_ip = match common::setting::get_config().guest.as_ref().and_then(|guests| guests.first()) {
        Some(guest) => guest.ip.clone(),
        None => {
            return Err(Status::unavailable("No guest configuration found"));
        }
    };
    
    send_to_node(action, guest_ip).await
}
