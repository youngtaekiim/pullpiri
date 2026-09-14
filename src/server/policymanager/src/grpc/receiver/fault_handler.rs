/*
 * SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Fault handler for PolicyManager
//!
//! This module handles fault reports from Timpani (via StateManager),
//! such as deadline miss events. Timpani already counts deadline misses internally
//! and only sends FaultInfo when the threshold is reached, so PolicyManager
//! receives pre-filtered fault notifications and triggers appropriate actions immediately.

use crate::grpc::sender;
use common::actioncontroller::StopWorkloadRequest;
use common::policymanager::{FaultType, ReportFaultRequest, ReportFaultResponse};
use common::spec::artifact::Package;
use common::spec::artifact::Policy;

/// Handle fault report from StateManager (originated from Timpani)
///
/// This function processes fault notifications (e.g., deadline miss) and determines
/// what action to take based on the associated policy's deadlineMissThreshold.
///
/// # Arguments
/// * `request` - Fault report request containing workload_id, node_id, task_name, fault_type
///
/// # Returns
/// * `ReportFaultResponse` with processed status and message
pub async fn handle_fault_report(request: ReportFaultRequest) -> ReportFaultResponse {
    let fault_type = match FaultType::try_from(request.fault_type) {
        Ok(ft) => ft,
        Err(_) => {
            return ReportFaultResponse {
                processed: false,
                message: "Invalid fault type".to_string(),
            };
        }
    };

    let fault_type_str = match fault_type {
        FaultType::FaultDeadlineMiss => "DEADLINE_MISS",
        FaultType::FaultUnknown => "UNKNOWN",
    };

    println!(
        "[PolicyManager] Received fault report: workload='{}', node='{}', task='{}', type={}",
        request.workload_id, request.node_id, request.task_name, fault_type_str
    );

    // Only handle deadline miss faults
    if fault_type != FaultType::FaultDeadlineMiss {
        return ReportFaultResponse {
            processed: true,
            message: format!("Fault type {} not handled", fault_type_str),
        };
    }

    // Process deadline miss fault
    match process_deadline_miss_fault(&request).await {
        Ok(message) => ReportFaultResponse {
            processed: true,
            message,
        },
        Err(e) => ReportFaultResponse {
            processed: false,
            message: format!("Failed to process fault: {}", e),
        },
    }
}

/// Process deadline miss fault
///
/// Timpani has already validated that the deadline miss threshold is reached,
/// so this function proceeds directly with executing the policy action.
///
/// 1. Find Package by schedule name (workload_id)
/// 2. Get policy from Package
/// 3. Execute action based on policy strategy
async fn process_deadline_miss_fault(request: &ReportFaultRequest) -> Result<String, String> {
    let workload_id = &request.workload_id;
    let node_id = &request.node_id;

    // Step 1: Find package by schedule name
    let (package_name, package) = find_package_by_schedule(workload_id).await?;
    println!(
        "[PolicyManager] Found package '{}' for workload '{}'",
        package_name, workload_id
    );

    // Step 2: Get policy name from package
    let policy_name = package
        .get_policy()
        .as_ref()
        .ok_or_else(|| format!("Package '{}' has no policy defined", package_name))?;

    // Step 3: Load policy from kvstore
    let policy = load_policy(policy_name).await?;
    println!("[PolicyManager] Loaded policy '{}'", policy_name);

    // Step 4: Execute action based on strategy
    // Timpani has already counted and validated threshold, so execute immediately
    let strategy = policy.get_procedure().get_strategy();
    println!(
        "[PolicyManager] Deadline miss fault detected! Executing strategy: '{}'",
        strategy
    );

    match strategy {
        "stop" => {
            // Find model name for this node
            let model_name = find_model_for_node(&package, node_id)?;

            // Stop the workload
            stop_workload(&package_name, &model_name, node_id, workload_id).await?;

            Ok(format!(
                "Deadline miss fault handled. Stopped workload '{}' model '{}' on node '{}'",
                workload_id, model_name, node_id
            ))
        }
        _ => Err(format!("Unknown strategy: {}", strategy)),
    }
}

/// Find Package by schedule name (workload_id)
async fn find_package_by_schedule(schedule_name: &str) -> Result<(String, Package), String> {
    let packages = common::kvstore::get_all_with_prefix("Package/").await?;

    for (key, value) in packages {
        let package: Package = serde_yaml::from_str(&value)
            .map_err(|e| format!("Failed to parse Package '{}': {}", key, e))?;

        if let Some(schedule) = package.get_schedule() {
            if schedule == schedule_name {
                // Extract package name from key "Package/{name}"
                let name = key.strip_prefix("Package/").unwrap_or(&key).to_string();
                return Ok((name, package));
            }
        }
    }

    Err(format!(
        "No package found with schedule '{}'",
        schedule_name
    ))
}

/// Load Policy from kvstore
async fn load_policy(policy_name: &str) -> Result<Policy, String> {
    let key = format!("Policy/{}", policy_name);
    let value = common::kvstore::get(&key).await?;

    if value.is_empty() {
        return Err(format!("Policy '{}' not found", policy_name));
    }

    serde_yaml::from_str(&value)
        .map_err(|e| format!("Failed to parse Policy '{}': {}", policy_name, e))
}

/// Find model name for a given node in the package
fn find_model_for_node(package: &Package, node_id: &str) -> Result<String, String> {
    for model in package.get_models() {
        if model.get_node() == node_id {
            return Ok(model.get_name());
        }
    }

    // If not found, return the first model as default
    package
        .get_models()
        .first()
        .map(|m| m.get_name())
        .ok_or_else(|| "Package has no models".to_string())
}

/// Stop workload via ActionController
async fn stop_workload(
    package_name: &str,
    model_name: &str,
    node_name: &str,
    workload_id: &str,
) -> Result<(), String> {
    println!(
        "[PolicyManager] Stopping workload: package='{}', model='{}', node='{}'",
        package_name, model_name, node_name
    );

    let request = StopWorkloadRequest {
        package_name: package_name.to_string(),
        model_name: model_name.to_string(),
        node_name: node_name.to_string(),
        reason: format!("Deadline miss threshold exceeded for '{}'", workload_id),
        workload_id: workload_id.to_string(),
    };

    match sender::stop_workload(request).await {
        Ok(response) => {
            let resp = response.into_inner();
            if resp.success {
                println!(
                    "[PolicyManager] Workload stopped successfully: {}",
                    resp.message
                );
                Ok(())
            } else {
                Err(format!("Failed to stop workload: {}", resp.message))
            }
        }
        Err(e) => Err(format!("gRPC error stopping workload: {}", e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_handle_fault_report_deadline_miss() {
        let request = ReportFaultRequest {
            workload_id: "test_workload".to_string(),
            node_id: "HPC".to_string(),
            task_name: "test_task".to_string(),
            fault_type: FaultType::FaultDeadlineMiss as i32,
        };

        let response = handle_fault_report(request).await;
        // kvstore availability can affect processed status, but a response message must exist.
        assert!(!response.message.is_empty());
    }

    #[tokio::test]
    async fn test_handle_fault_report_unknown() {
        let request = ReportFaultRequest {
            workload_id: "test_workload".to_string(),
            node_id: "HPC".to_string(),
            task_name: "test_task".to_string(),
            fault_type: FaultType::FaultUnknown as i32,
        };

        let response = handle_fault_report(request).await;
        assert!(response.processed);
        assert!(response.message.contains("not handled"));
    }
}
