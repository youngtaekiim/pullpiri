/*
 * SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
 * SPDX-License-Identifier: Apache-2.0
 */

use common::policymanager::policy_manager_connection_server::PolicyManagerConnection;
use common::policymanager::{CheckNodePolicyRequest, CheckNodePolicyResponse};
use common::spec::artifact::Policy;
use tonic::{Request, Response, Status};

const ETCD_POLICY_PREFIX: &str = "Policy";

/// gRPC server implementation for PolicyManager
pub struct PolicyManagerGrpcServer {}

impl PolicyManagerGrpcServer {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for PolicyManagerGrpcServer {
    fn default() -> Self {
        Self::new()
    }
}

#[tonic::async_trait]
impl PolicyManagerConnection for PolicyManagerGrpcServer {
    /// Check if deployment to a specific node is allowed based on policy
    ///
    /// # Arguments
    /// * `request` - Contains policy_name and target_node
    ///
    /// # Returns
    /// * `CheckNodePolicyResponse` - allowed status, fallback_node, and message
    async fn check_node_policy(
        &self,
        request: Request<CheckNodePolicyRequest>,
    ) -> Result<Response<CheckNodePolicyResponse>, Status> {
        let req = request.into_inner();
        let policy_name = req.policy_name;
        let target_node = req.target_node;

        println!(
            "PolicyManager: Checking policy '{}' for node '{}'",
            policy_name, target_node
        );

        // If no policy specified, allow by default
        if policy_name.is_empty() {
            println!("No policy specified, allowing deployment");
            return Ok(Response::new(CheckNodePolicyResponse {
                allowed: true,
                fallback_node: String::new(),
                message: "No policy specified, deployment allowed".to_string(),
            }));
        }

        // Fetch policy from etcd
        let etcd_key = format!("{}/{}", ETCD_POLICY_PREFIX, policy_name);
        let policy_str = match common::etcd::get(&etcd_key).await {
            Ok(s) => s,
            Err(e) => {
                println!("Policy '{}' not found in etcd: {}", policy_name, e);
                // If policy not found, allow by default (fail-open)
                return Ok(Response::new(CheckNodePolicyResponse {
                    allowed: true,
                    fallback_node: String::new(),
                    message: format!("Policy '{}' not found, allowing deployment", policy_name),
                }));
            }
        };

        // Parse policy
        let policy: Policy = match serde_yaml::from_str(&policy_str) {
            Ok(p) => p,
            Err(e) => {
                println!("Failed to parse policy '{}': {}", policy_name, e);
                return Err(Status::internal(format!(
                    "Failed to parse policy '{}': {}",
                    policy_name, e
                )));
            }
        };

        // Check if target_node is in availableNodes
        let placement = policy.get_placement();
        let available_nodes = placement.get_available_nodes();
        let preferred_node = placement.get_preferred_node().unwrap_or("").to_string();

        let allowed = available_nodes.contains(&target_node);

        if allowed {
            println!(
                "Node '{}' is in availableNodes {:?}",
                target_node, available_nodes
            );
        } else {
            println!(
                "Node '{}' is NOT in availableNodes {:?}",
                target_node, available_nodes
            );
            if !preferred_node.is_empty() {
                println!("Suggested preferred node: '{}'", preferred_node);
            }
        }

        Ok(Response::new(CheckNodePolicyResponse {
            allowed,
            fallback_node: preferred_node,
            message: if allowed {
                format!(
                    "Node '{}' is allowed by policy '{}'",
                    target_node, policy_name
                )
            } else {
                format!(
                    "Node '{}' is not in availableNodes {:?}",
                    target_node, available_nodes
                )
            },
        }))
    }
}
