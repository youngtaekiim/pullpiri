use common::nodeagent::{
    node_agent_connection_client::NodeAgentConnectionClient,
    HandleYamlRequest, HandleYamlResponse,
};
use tonic::{Request, Response, Status};

// Send to a specific node using its IP address
pub async fn send_to_node(action: HandleYamlRequest, node_ip: String) -> Result<Response<HandleYamlResponse>, Status> {
    let addr = format!("http://{}:47004", node_ip);
    
    println!("Attempting to connect to NodeAgent at: {}", addr);
    
    // Attempting to connect with a timeout
    let client_result = tokio::time::timeout(
        std::time::Duration::from_secs(5), 
        NodeAgentConnectionClient::connect(addr.clone())
    ).await;
    
    match client_result {
        Ok(Ok(mut client)) => {
            println!("Successfully connected to NodeAgent, sending request...");
            match tokio::time::timeout(
                std::time::Duration::from_secs(10),
                client.handle_yaml(Request::new(action))
            ).await {
                Ok(result) => {
                    match result {
                        Ok(response) => {
                            println!("Request to NodeAgent successful");
                            Ok(response)
                        },
                        Err(e) => {
                            eprintln!("Error calling NodeAgent handle_yaml: {}", e);
                            Err(Status::internal(format!("Error calling NodeAgent handle_yaml: {}", e)))
                        }
                    }
                },
                Err(_) => {
                    eprintln!("Timeout while waiting for NodeAgent to respond");
                    Err(Status::deadline_exceeded("Timeout while waiting for NodeAgent to respond"))
                }
            }
        },
        Ok(Err(e)) => {
            eprintln!("Error connecting to NodeAgent at {}: {}", addr, e);
            eprintln!("Connection error details: {:?}", e);
            Err(Status::unavailable(format!("Failed to connect to NodeAgent at {}: {}", addr, e)))
        },
        Err(_) => {
            eprintln!("Timeout while connecting to NodeAgent at {}", addr);
            Err(Status::deadline_exceeded(format!("Timeout while connecting to NodeAgent at {}", addr)))
        }
    }
}

pub async fn send(action: HandleYamlRequest) -> Result<Response<HandleYamlResponse>, Status> {
    // Use the node lookup module to get the node IP
    let node_ip = crate::node::get_node_ip().await;
    
    // Send to the node
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
