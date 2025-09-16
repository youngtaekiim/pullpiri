/*
 * SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
 * SPDX-License-Identifier: Apache-2.0
 */

use common::apiserver::api_server_connection_server::ApiServerConnection;
use common::apiserver::{
    ClusterTopology, GetNodeRequest, GetNodeResponse, GetNodesRequest, GetNodesResponse,
    GetTopologyRequest, GetTopologyResponse, NodeInfo, TopologyType, UpdateTopologyRequest,
    UpdateTopologyResponse,
};
use common::etcd;
use common::nodeagent::{NodeRegistrationRequest, NodeRegistrationResponse, NodeStatus};
use prost::Message;
use tonic::{Request, Response, Status};
use crate::node::NodeManager;

/// Simple registry embedded in receiver
#[derive(Clone)]
struct NodeRegistry;

impl NodeRegistry {
    async fn get_topology(
        &self,
    ) -> Result<ClusterTopology, Box<dyn std::error::Error + Send + Sync>> {
        let topology_key = "cluster/topology";

        match etcd::get(topology_key).await {
            Ok(encoded) => {
                let buf = base64::decode(&encoded)?;
                let topology = ClusterTopology::decode(&buf[..])?;
                Ok(topology)
            }
            Err(_) => Ok(ClusterTopology {
                cluster_id: "default-cluster".to_string(),
                cluster_name: "PICCOLO Cluster".to_string(),
                r#type: TopologyType::Embedded.into(),
                master_nodes: vec![],
                sub_nodes: vec![],
                parent_cluster: String::new(),
                config: std::collections::HashMap::new(),
            }),
        }
    }

    async fn update_topology(
        &self,
        topology: ClusterTopology,
    ) -> Result<ClusterTopology, Box<dyn std::error::Error + Send + Sync>> {
        let topology_key = "cluster/topology";

        let mut buf = Vec::new();
        prost::Message::encode(&topology, &mut buf)?;
        let encoded = base64::encode(&buf);

        etcd::put(topology_key, &encoded).await?;

        println!("Updated cluster topology: {}", topology.cluster_name);
        Ok(topology)
    }
}

/// API Server gRPC service handler for clustering functionality
#[derive(Clone)]
pub struct ApiServerReceiver {
    node_manager: NodeManager,
    registry: NodeRegistry,
}

impl ApiServerReceiver {
    pub fn new() -> Self {
        Self {
            node_manager: NodeManager::new().unwrap(),
            registry: NodeRegistry,
        }
    }

    /// Convert to gRPC service
    pub fn into_service(
        self,
    ) -> common::apiserver::api_server_connection_server::ApiServerConnectionServer<Self> {
        common::apiserver::api_server_connection_server::ApiServerConnectionServer::new(self)
    }
}

impl Default for ApiServerReceiver {
    fn default() -> Self {
        Self::new()
    }
}

#[tonic::async_trait]
impl ApiServerConnection for ApiServerReceiver {
    /// Get all nodes in the cluster
    async fn get_nodes(
        &self,
        request: Request<GetNodesRequest>,
    ) -> Result<Response<GetNodesResponse>, Status> {
        println!("Received GetNodes request");
        let _req = request.into_inner();

        match self.node_manager.get_all_nodes().await {
            Ok(nodes) => {
                let response = GetNodesResponse {
                    nodes,
                    success: true,
                    message: "Nodes retrieved successfully".to_string(),
                };
                Ok(Response::new(response))
            }
            Err(e) => {
                let response = GetNodesResponse {
                    nodes: vec![],
                    success: false,
                    message: format!("Failed to retrieve nodes: {}", e),
                };
                Ok(Response::new(response))
            }
        }
    }

    /// Get a specific node by ID
    async fn get_node(
        &self,
        request: Request<GetNodeRequest>,
    ) -> Result<Response<GetNodeResponse>, Status> {
        println!("Received GetNode request");
        let req = request.into_inner();

        match self.node_manager.get_node(&req.node_id).await {
            Ok(Some(node)) => {
                let response = GetNodeResponse {
                    node: Some(node),
                    success: true,
                    message: "Node retrieved successfully".to_string(),
                };
                Ok(Response::new(response))
            }
            Ok(None) => {
                let response = GetNodeResponse {
                    node: None,
                    success: false,
                    message: format!("Node {} not found", req.node_id),
                };
                Ok(Response::new(response))
            }
            Err(e) => {
                let response = GetNodeResponse {
                    node: None,
                    success: false,
                    message: format!("Failed to retrieve node: {}", e),
                };
                Ok(Response::new(response))
            }
        }
    }

    /// Register a new node in the cluster
    async fn register_node(
        &self,
        request: Request<NodeRegistrationRequest>,
    ) -> Result<Response<NodeRegistrationResponse>, Status> {
        println!("Received RegisterNode request");
        let mut req = request.into_inner();

        println!("Registering node: {} ({})", req.hostname, req.ip_address);
        
        // Simplify the node_id to make it easier to search for
        req.node_id = format!("node-{}", req.ip_address.replace(".", "-"));
        println!("Using simplified node_id: {}", req.node_id);

        // Let's update the node status to Ready immediately
        match self.node_manager.register_node(req.clone()).await {
            Ok(cluster_token) => {
                // Also directly add to etcd with a simple key
                let node_info = common::apiserver::NodeInfo {
                    node_id: req.node_id.clone(),
                    hostname: req.hostname.clone(),
                    ip_address: req.ip_address.clone(),
                    role: req.role,
                    status: NodeStatus::Ready.into(),
                    resources: req.resources.clone(),
                    last_heartbeat: chrono::Utc::now().timestamp(),
                    created_at: chrono::Utc::now().timestamp(),
                    metadata: req.metadata.clone(),
                };

                let mut buf = Vec::new();
                prost::Message::encode(&node_info, &mut buf).unwrap();
                let encoded = base64::encode(&buf);
                let _ = common::etcd::put(&format!("nodes/{}", req.ip_address), &encoded).await;
                
                println!("Node info also stored at simple key: nodes/{}", req.ip_address);

                // Immediately update the node status to Ready
                if let Err(e) = self.node_manager.update_status(&req.node_id, NodeStatus::Ready).await {
                    println!("Warning: Failed to update node status to Ready: {}", e);
                } else {
                    println!("Successfully updated node status to Ready");
                }

                let response = NodeRegistrationResponse {
                    success: true,
                    message: "Node registered successfully".to_string(),
                    cluster_token,
                    cluster_config: Some(common::nodeagent::ClusterConfig {
                        master_endpoint: common::apiserver::connect_grpc_server(),
                        heartbeat_interval: 30,
                        settings: {
                            let mut settings = std::collections::HashMap::new();
                            settings
                                .insert("cluster_id".to_string(), "default-cluster".to_string());
                            settings
                        },
                    }),
                };
                Ok(Response::new(response))
            }
            Err(e) => {
                let response = NodeRegistrationResponse {
                    success: false,
                    message: format!("Node registration failed: {}", e),
                    cluster_token: String::new(),
                    cluster_config: None,
                };
                Ok(Response::new(response))
            }
        }
    }

    /// Get the current cluster topology
    async fn get_topology(
        &self,
        request: Request<GetTopologyRequest>,
    ) -> Result<Response<GetTopologyResponse>, Status> {
        println!("Received GetTopology request");
        let _req = request.into_inner();

        match self.registry.get_topology().await {
            Ok(topology) => {
                let response = GetTopologyResponse {
                    topology: Some(topology),
                    success: true,
                    message: "Topology retrieved successfully".to_string(),
                };
                Ok(Response::new(response))
            }
            Err(e) => {
                let response = GetTopologyResponse {
                    topology: None,
                    success: false,
                    message: format!("Failed to retrieve topology: {}", e),
                };
                Ok(Response::new(response))
            }
        }
    }

    /// Update the cluster topology
    async fn update_topology(
        &self,
        request: Request<UpdateTopologyRequest>,
    ) -> Result<Response<UpdateTopologyResponse>, Status> {
        println!("Received UpdateTopology request");
        let req = request.into_inner();

        if let Some(topology) = req.topology {
            match self.registry.update_topology(topology).await {
                Ok(updated_topology) => {
                    let response = UpdateTopologyResponse {
                        updated_topology: Some(updated_topology),
                        success: true,
                        message: "Topology updated successfully".to_string(),
                    };
                    Ok(Response::new(response))
                }
                Err(e) => {
                    let response = UpdateTopologyResponse {
                        updated_topology: None,
                        success: false,
                        message: format!("Failed to update topology: {}", e),
                    };
                    Ok(Response::new(response))
                }
            }
        } else {
            let response = UpdateTopologyResponse {
                updated_topology: None,
                success: false,
                message: "No topology provided in request".to_string(),
            };
            Ok(Response::new(response))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tonic::Request;

    #[tokio::test]
    async fn test_get_nodes() {
        let receiver = ApiServerReceiver::new();
        let request = Request::new(GetNodesRequest {
            filter: None,
            status_filter: None,
        });

        let response = receiver.get_nodes(request).await;
        assert!(response.is_ok());

        let response = response.unwrap().into_inner();
        assert!(response.success);
        // Don't assert on specific count as etcd might have data from previous runs
        // Just ensure we get a valid response
    }

    #[tokio::test]
    async fn test_register_node() {
        let receiver = ApiServerReceiver::new();
        let request = Request::new(NodeRegistrationRequest {
            node_id: "test-node".to_string(),
            hostname: "test-host".to_string(),
            ip_address: "192.168.1.100".to_string(),
            role: common::nodeagent::NodeRole::Sub.into(),
            resources: Some(common::nodeagent::ResourceInfo {
                cpu_cores: 4,
                memory_mb: 8192,
                disk_gb: 100,
                architecture: "x86_64".to_string(),
                os_version: "Ubuntu 20.04".to_string(),
            }),
            metadata: std::collections::HashMap::new(),
        });

        let response = receiver.register_node(request).await;
        assert!(response.is_ok());

        let response = response.unwrap().into_inner();
        assert!(response.success);
        assert!(!response.cluster_token.is_empty());
    }
}
