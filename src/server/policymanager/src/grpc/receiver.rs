/*
 * SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
 * SPDX-License-Identifier: Apache-2.0
 */

use common::policymanager::policy_manager_connection_server::PolicyManagerConnection;
use common::policymanager::{
    CheckNodePolicyRequest, CheckNodePolicyResponse, ReportNodeMetricsRequest,
    ReportNodeMetricsResponse, RunningContainer,
};
use common::spec::artifact::Policy;
use common::statemanager::OffloadingRequest;
use tonic::{Request, Response, Status};

const ETCD_POLICY_PREFIX: &str = "Policy";

/// gRPC server implementation for PolicyManager
pub struct PolicyManagerGrpcServer {}

impl PolicyManagerGrpcServer {
    pub fn new() -> Self {
        Self {}
    }

    /// Check if resource threshold is exceeded and trigger offloading if needed
    async fn check_threshold_and_trigger_offloading(
        &self,
        node_info: &common::monitoringserver::NodeInfo,
        container: &RunningContainer,
    ) {
        let policy_name = &container.policy_name;
        if policy_name.is_empty() {
            return;
        }

        // Fetch policy from etcd
        let etcd_key = format!("{}/{}", ETCD_POLICY_PREFIX, policy_name);
        let policy_str = match common::etcd::get(&etcd_key).await {
            Ok(s) => s,
            Err(_) => return, // Policy not found, skip
        };

        // Parse policy
        let policy: Policy = match serde_yaml::from_str(&policy_str) {
            Ok(p) => p,
            Err(_) => return, // Parse error, skip
        };

        // Get threshold from policy
        let procedure = policy.get_procedure();
        let trigger = procedure.get_trigger();

        let threshold = match &trigger.resourceThreshold {
            Some(t) => t,
            None => return, // No threshold defined
        };

        // Check CPU threshold
        let cpu_exceeded = threshold.cpu.map_or(false, |cpu_threshold| {
            node_info.cpu_usage > cpu_threshold as f64
        });

        // Check memory threshold
        let mem_exceeded = threshold.memory.map_or(false, |mem_threshold| {
            node_info.mem_usage > mem_threshold as f64
        });

        if !cpu_exceeded && !mem_exceeded {
            return; // No threshold exceeded
        }

        // Find target node for offloading
        let placement = policy.get_placement();
        let available_nodes = placement.get_available_nodes();
        let current_node = &node_info.node_name;

        // Find first available node that is not the current node
        let target_node = available_nodes.iter().find(|n| *n != current_node).cloned();

        let target_node = match target_node {
            Some(n) => n,
            None => {
                println!(
                    "[PolicyManager] No target node available for offloading container '{}' from '{}'",
                    container.container_name, current_node
                );
                return;
            }
        };

        // Build reason message
        let reason = if cpu_exceeded && mem_exceeded {
            format!(
                "CPU ({:.1}% > {}%) and Memory ({:.1}% > {}%) threshold exceeded",
                node_info.cpu_usage,
                threshold.cpu.unwrap_or(0),
                node_info.mem_usage,
                threshold.memory.unwrap_or(0)
            )
        } else if cpu_exceeded {
            format!(
                "CPU threshold exceeded: {:.1}% > {}%",
                node_info.cpu_usage,
                threshold.cpu.unwrap_or(0)
            )
        } else {
            format!(
                "Memory threshold exceeded: {:.1}% > {}%",
                node_info.mem_usage,
                threshold.memory.unwrap_or(0)
            )
        };

        println!(
            "[PolicyManager] Triggering offloading: container '{}' from '{}' to '{}'. Reason: {}",
            container.container_name, current_node, target_node, reason
        );

        // Trigger offloading via StateManager
        let offloading_request = OffloadingRequest {
            scenario_name: container.scenario_name.clone(),
            package_name: container.package_name.clone(),
            model_name: container.container_name.clone(),
            source_node: current_node.clone(),
            target_node: target_node.clone(),
            policy_name: policy_name.clone(),
            reason,
        };

        match super::sender::trigger_offloading(offloading_request).await {
            Ok(response) => {
                let resp = response.into_inner();
                if resp.accepted {
                    println!(
                        "[PolicyManager] Offloading request accepted: {}",
                        resp.message
                    );
                } else {
                    println!(
                        "[PolicyManager] Offloading request rejected: {}",
                        resp.message
                    );
                }
            }
            Err(e) => {
                eprintln!(
                    "[PolicyManager] Failed to trigger offloading: {}",
                    e.message()
                );
            }
        }
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
            "[PolicyManager] Checking policy '{}' for node '{}'",
            policy_name, target_node
        );

        // If no policy specified, allow by default
        if policy_name.is_empty() {
            println!("[PolicyManager] No policy specified, allowing deployment");
            return Ok(Response::new(CheckNodePolicyResponse {
                allowed: true,
                suggested_node: String::new(),
                message: "No policy specified, deployment allowed".to_string(),
            }));
        }

        // Fetch policy from etcd
        let etcd_key = format!("{}/{}", ETCD_POLICY_PREFIX, policy_name);
        let policy_str = match common::etcd::get(&etcd_key).await {
            Ok(s) => s,
            Err(e) => {
                println!(
                    "[PolicyManager] Policy '{}' not found in etcd: {}",
                    policy_name, e
                );
                // If policy not found, allow by default (fail-open)
                return Ok(Response::new(CheckNodePolicyResponse {
                    allowed: true,
                    suggested_node: String::new(),
                    message: format!("Policy '{}' not found, allowing deployment", policy_name),
                }));
            }
        };

        // Parse policy
        let policy: Policy = match serde_yaml::from_str(&policy_str) {
            Ok(p) => p,
            Err(e) => {
                println!(
                    "[PolicyManager] Failed to parse policy '{}': {}",
                    policy_name, e
                );
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
                "[PolicyManager] Node '{}' is in availableNodes {:?}",
                target_node, available_nodes
            );
        } else {
            println!(
                "[PolicyManager] Node '{}' is NOT in availableNodes {:?}",
                target_node, available_nodes
            );
            if !preferred_node.is_empty() {
                println!(
                    "[PolicyManager] Suggested preferred node: '{}'",
                    preferred_node
                );
            }
        }

        Ok(Response::new(CheckNodePolicyResponse {
            allowed,
            suggested_node: preferred_node,
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

    /// Report node metrics from monitoring server for threshold-based policy evaluation
    ///
    /// This method is called by MonitoringServer whenever NodeInfo is received.
    /// It checks if any running containers have policies with resource thresholds,
    /// and triggers offloading if thresholds are exceeded.
    async fn report_node_metrics(
        &self,
        request: Request<ReportNodeMetricsRequest>,
    ) -> Result<Response<ReportNodeMetricsResponse>, Status> {
        let req = request.into_inner();

        let node_info = match req.node_info {
            Some(info) => info,
            None => {
                return Ok(Response::new(ReportNodeMetricsResponse {
                    processed: false,
                    message: "No NodeInfo provided".to_string(),
                }));
            }
        };

        let running_containers = req.running_containers;

        println!(
            "[PolicyManager] Received metrics for node '{}': CPU={:.1}%, Mem={:.1}%, Containers={}",
            node_info.node_name,
            node_info.cpu_usage,
            node_info.mem_usage,
            running_containers.len()
        );

        // Check each container with a policy for threshold violations
        for container in &running_containers {
            if !container.policy_name.is_empty() {
                self.check_threshold_and_trigger_offloading(&node_info, container)
                    .await;
            }
        }

        Ok(Response::new(ReportNodeMetricsResponse {
            processed: true,
            message: format!(
                "Processed metrics for node '{}' with {} containers",
                node_info.node_name,
                running_containers.len()
            ),
        }))
    }
}
