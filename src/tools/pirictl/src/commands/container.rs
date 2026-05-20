/*
* SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
* SPDX-License-Identifier: Apache-2.0
*/
use crate::commands::format::{
    calculate_age, calculate_runtime, calculate_uptime, capitalize, extract_network_value,
    format_bytes, format_timestamp,
};
use crate::commands::{print_error, print_info, print_success, print_table_header};
use crate::{Result, SettingsClient};
use clap::Subcommand;
use colored::Colorize;

#[derive(Subcommand)]
pub enum ContainerAction {
    /// Get all containers
    Get,
    /// Describe specific container by ID
    Describe { id: String },
    /// Get container information in raw JSON format
    Raw,
}

pub async fn handle(client: &SettingsClient, action: ContainerAction) -> Result<()> {
    match action {
        ContainerAction::Get => get_containers(client).await,
        ContainerAction::Describe { id } => describe_container(client, &id).await,
        ContainerAction::Raw => raw_containers(client).await,
    }
}

/// Get all containers
async fn get_containers(client: &SettingsClient) -> Result<()> {
    print_info("Fetching containers list...");

    match client.get("/api/v1/containers").await {
        Ok(containers) => {
            // If the response is an array, iterate and print each container
            let containers_array = if let Some(array) = containers.as_array() {
                array
            } else if let Some(array) = containers.get("containers").and_then(|c| c.as_array()) {
                array
            } else {
                println!("No containers found.");
                print_success("Containers list retrieved successfully");
                return Ok(());
            };

            if containers_array.is_empty() {
                println!("No containers found.");
            } else {
                print_table_header(
                    "Containers",
                    &[("NAME", 32), ("STATUS", 12), ("ID", 66), ("AGE", 8)],
                );

                // Print each container
                for container in containers_array.iter() {
                    let name = container
                        .get("names")
                        .and_then(|n| n.as_array())
                        .and_then(|arr| arr.first())
                        .and_then(|n| n.as_str())
                        .unwrap_or("Unknown");

                    let status = container
                        .get("state")
                        .and_then(|s| s.get("Status"))
                        .and_then(|st| st.as_str())
                        .unwrap_or("Unknown");

                    let id = container
                        .get("id")
                        .and_then(|i| i.as_str())
                        .unwrap_or("Unknown");

                    let age = container
                        .get("state")
                        .and_then(|s| s.get("StartedAt"))
                        .and_then(|t| t.as_str())
                        .and_then(|t| calculate_age(t).ok())
                        .unwrap_or_else(|| "N/A".to_string());

                    println!("{:<32} {:<12} {:<66} {:<8}", name, status, id, age);
                }
            }

            println!();
            print_success("Containers list retrieved successfully");
        }
        Err(e) => {
            print_error(&format!("Failed to fetch containers: {}", e));
            return Err(e.into());
        }
    }

    Ok(())
}

/// Describe specific container information
async fn describe_container(client: &SettingsClient, container_id: &str) -> Result<()> {
    print_info(&format!(
        "Fetching detailed container information for ID: {}",
        container_id
    ));

    let endpoint = format!("/api/v1/containers/{}", container_id);
    match client.get(&endpoint).await {
        Ok(container) => {
            // Get container name
            let container_name = container
                .get("names")
                .and_then(|n| n.as_array())
                .and_then(|arr| arr.first())
                .and_then(|n| n.as_str())
                .unwrap_or(container_id);

            println!("\n{:<24}{}", format!("{}:", "Name".bold()), container_name);

            // Node (from config.Hostname)
            if let Some(hostname) = container
                .get("config")
                .and_then(|c| c.get("Hostname"))
                .and_then(|h| h.as_str())
            {
                println!("{:<24}{}", format!("{}:", "Node".bold()), hostname);
            }

            // Status
            let status = container
                .get("state")
                .and_then(|s| s.get("Status"))
                .and_then(|st| st.as_str())
                .unwrap_or("Unknown");

            let is_running = container
                .get("state")
                .and_then(|s| s.get("Running"))
                .and_then(|r| r.as_str())
                .map(|r| r == "true")
                .unwrap_or(false);

            println!(
                "{:<24}{}",
                format!("{}:", "Status".bold()),
                if is_running { "Running" } else { status }
            );

            // Container section
            println!("{}", "Container:".bold());

            // ID
            if let Some(id) = container.get("id").and_then(|i| i.as_str()) {
                println!("  {:<22}{}", "ID:", id);
            }

            // Image
            if let Some(image) = container.get("image").and_then(|i| i.as_str()) {
                println!("  {:<22}{}", "Image:", image);
            }

            // State information
            if let Some(state) = container.get("state") {
                let state_status = state
                    .get("Status")
                    .and_then(|s| s.as_str())
                    .unwrap_or("Unknown");
                println!("  {:<22}{}", "State:", capitalize(state_status));

                if is_running {
                    // Running container
                    if let Some(started_at) = state.get("StartedAt").and_then(|s| s.as_str()) {
                        if let Ok(formatted) = format_timestamp(started_at) {
                            println!("  {:<22}{}", "Started:", formatted);
                        }
                        // Calculate uptime
                        if let Ok(uptime) = calculate_uptime(started_at) {
                            println!("  {:<22}{}", "Uptime:", uptime);
                        }
                    }

                    if let Some(pid) = state.get("Pid").and_then(|p| p.as_str()) {
                        println!("  {:<22}{}", "PID:", pid);
                    }

                    println!(
                        "  {:<22}{}",
                        "Ready:",
                        if is_running { "True" } else { "False" }
                    );
                } else {
                    // Terminated container
                    let exit_code = state
                        .get("ExitCode")
                        .and_then(|e| e.as_str())
                        .unwrap_or("0");
                    let reason = if exit_code == "0" {
                        "Completed"
                    } else {
                        "Error"
                    };
                    println!("  {:<22}{}", "Reason:", reason);
                    println!("  {:<22}{}", "Exit Code:", exit_code);

                    if let Some(started_at) = state.get("StartedAt").and_then(|s| s.as_str()) {
                        if let Ok(formatted) = format_timestamp(started_at) {
                            println!("  {:<22}{}", "Started:", formatted);
                        }
                    }

                    if let Some(finished_at) = state.get("FinishedAt").and_then(|f| f.as_str()) {
                        if let Ok(formatted) = format_timestamp(finished_at) {
                            println!("  {:<22}{}", "Finished:", formatted);
                        }

                        // Calculate runtime
                        if let (Some(started), Some(finished)) = (
                            state.get("StartedAt").and_then(|s| s.as_str()),
                            state.get("FinishedAt").and_then(|f| f.as_str()),
                        ) {
                            if let Ok(runtime) = calculate_runtime(started, finished) {
                                println!("  {:<22}{}", "Runtime:", runtime);
                            }
                        }
                    }

                    let oom_killed = state
                        .get("OOMKilled")
                        .and_then(|o| o.as_str())
                        .unwrap_or("false");
                    println!("  {:<22}{}", "OOMKilled:", oom_killed);
                    println!("  {:<22}False", "Ready:");
                }
            }

            // Resource Usage
            println!("{}", "Resource Usage:".bold());

            if is_running {
                if let Some(stats) = container.get("stats") {
                    // Check if stats are available
                    if stats.get("Status").and_then(|s| s.as_str()) == Some("StatsUnavailable") {
                        println!("  {:<22}N/A (stats unavailable)", "");
                    } else {
                        // CPU Usage
                        if let (Some(total_cpu), Some(kernel_cpu), Some(user_cpu)) = (
                            stats
                                .get("CpuTotalUsage")
                                .and_then(|c| c.as_str())
                                .and_then(|s| s.parse::<f64>().ok()),
                            stats
                                .get("CpuUsageInKernelMode")
                                .and_then(|c| c.as_str())
                                .and_then(|s| s.parse::<f64>().ok()),
                            stats
                                .get("CpuUsageInUserMode")
                                .and_then(|c| c.as_str())
                                .and_then(|s| s.parse::<f64>().ok()),
                        ) {
                            let total_secs = total_cpu / 1_000_000_000.0;
                            let kernel_secs = kernel_cpu / 1_000_000_000.0;
                            let user_secs = user_cpu / 1_000_000_000.0;
                            println!(
                                "  {:<22}{:.2}s ({:.2}s kernel, {:.2}s user)",
                                "CPU:", total_secs, kernel_secs, user_secs
                            );
                        }

                        // Memory Usage
                        if let (Some(mem_usage), Some(mem_limit)) = (
                            stats
                                .get("MemoryUsage")
                                .and_then(|m| m.as_str())
                                .and_then(|s| s.parse::<u64>().ok()),
                            stats
                                .get("MemoryLimit")
                                .and_then(|m| m.as_str())
                                .and_then(|s| s.parse::<u64>().ok()),
                        ) {
                            let usage_pct = if mem_limit > 0 {
                                (mem_usage as f64 / mem_limit as f64) * 100.0
                            } else {
                                0.0
                            };
                            println!(
                                "  {:<22}{} / {} ({:.2}%)",
                                "Memory:",
                                format_bytes(mem_usage),
                                format_bytes(mem_limit),
                                usage_pct
                            );
                        }

                        // Network Usage
                        if let Some(networks) = stats.get("Networks").and_then(|n| n.as_str()) {
                            let rx_bytes = extract_network_value(networks, "rx_bytes");
                            let tx_bytes = extract_network_value(networks, "tx_bytes");
                            println!(
                                "  {:<22}RX: {}, TX: {}",
                                "Network:",
                                format_bytes(rx_bytes),
                                format_bytes(tx_bytes)
                            );
                        }
                    }
                } else {
                    println!("  {:<22}N/A (stats unavailable)", "");
                }
            } else {
                println!("  {:<22}N/A (container exited)", "");
            }

            print_success("Container description retrieved successfully");
        }
        Err(e) => {
            print_error(&format!(
                "Failed to fetch container {}: {}",
                container_id, e
            ));
            return Err(e.into());
        }
    }

    Ok(())
}

/// Get raw containers data
async fn raw_containers(client: &SettingsClient) -> Result<()> {
    print_info("Fetching raw containers data...");

    match client.get("/api/v1/containers").await {
        Ok(containers) => {
            println!("{}", serde_json::to_string_pretty(&containers)?);
            print_success("Raw containers data retrieved successfully");
        }
        Err(e) => {
            print_error(&format!("Failed to fetch raw containers data: {}", e));
            return Err(e.into());
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    async fn make_client(base_url: &str) -> SettingsClient {
        SettingsClient::new(base_url, 5).unwrap()
    }

    // ── handle() dispatch ─────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_handle_get() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/containers"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"containers": []})))
            .mount(&server)
            .await;
        let client = make_client(&server.uri()).await;
        assert!(handle(&client, ContainerAction::Get).await.is_ok());
    }

    #[tokio::test]
    async fn test_handle_raw() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/containers"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!([])))
            .mount(&server)
            .await;
        let client = make_client(&server.uri()).await;
        assert!(handle(&client, ContainerAction::Raw).await.is_ok());
    }

    #[tokio::test]
    async fn test_handle_describe() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/containers/abc123"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "id": "abc123",
                "names": ["my-container"],
                "state": {"Status": "running", "Running": "false"}
            })))
            .mount(&server)
            .await;
        let client = make_client(&server.uri()).await;
        assert!(handle(
            &client,
            ContainerAction::Describe {
                id: "abc123".into()
            }
        )
        .await
        .is_ok());
    }

    // ── get_containers ────────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_get_containers_array_response() {
        let server = MockServer::start().await;
        let body = json!([
            {
                "names": ["container-a"],
                "state": {"Status": "running", "StartedAt": "2026-04-23T10:00:00+00:00"},
                "id": "aaaa1111"
            },
            {
                "names": ["container-b"],
                "state": {"Status": "exited", "StartedAt": "0001-01-01T00:00:00Z"},
                "id": "bbbb2222"
            }
        ]);
        Mock::given(method("GET"))
            .and(path("/api/v1/containers"))
            .respond_with(ResponseTemplate::new(200).set_body_json(body))
            .mount(&server)
            .await;
        let client = make_client(&server.uri()).await;
        assert!(handle(&client, ContainerAction::Get).await.is_ok());
    }

    #[tokio::test]
    async fn test_get_containers_with_containers_key() {
        let server = MockServer::start().await;
        let body = json!({"containers": [
            {"names": ["c1"], "state": {"Status": "running", "StartedAt": "2026-04-23T10:00:00+00:00"}, "id": "c1id"}
        ]});
        Mock::given(method("GET"))
            .and(path("/api/v1/containers"))
            .respond_with(ResponseTemplate::new(200).set_body_json(body))
            .mount(&server)
            .await;
        let client = make_client(&server.uri()).await;
        assert!(handle(&client, ContainerAction::Get).await.is_ok());
    }

    #[tokio::test]
    async fn test_get_containers_no_array_in_response() {
        // Response is a plain object with no array → "No containers found"
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/containers"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"status": "ok"})))
            .mount(&server)
            .await;
        let client = make_client(&server.uri()).await;
        assert!(handle(&client, ContainerAction::Get).await.is_ok());
    }

    #[tokio::test]
    async fn test_get_containers_empty_array() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/containers"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!([])))
            .mount(&server)
            .await;
        let client = make_client(&server.uri()).await;
        assert!(handle(&client, ContainerAction::Get).await.is_ok());
    }

    #[tokio::test]
    async fn test_get_containers_server_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/containers"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;
        let client = make_client(&server.uri()).await;
        assert!(handle(&client, ContainerAction::Get).await.is_err());
    }

    // ── describe_container ────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_describe_running_container_full_stats() {
        let server = MockServer::start().await;
        let body = json!({
            "id": "run-container-id",
            "names": ["running-app"],
            "image": "ubuntu:22.04",
            "config": {"Hostname": "my-host"},
            "state": {
                "Status": "running",
                "Running": "true",
                "StartedAt": "2026-04-23T10:00:00+00:00",
                "Pid": "12345"
            },
            "stats": {
                "CpuTotalUsage": "5000000000",
                "CpuUsageInKernelMode": "2000000000",
                "CpuUsageInUserMode": "3000000000",
                "MemoryUsage": "536870912",
                "MemoryLimit": "4294967296",
                "Networks": "network: {rx_bytes: 1024, tx_bytes: 2048}"
            }
        });
        Mock::given(method("GET"))
            .and(path("/api/v1/containers/run-container-id"))
            .respond_with(ResponseTemplate::new(200).set_body_json(body))
            .mount(&server)
            .await;
        let client = make_client(&server.uri()).await;
        assert!(handle(
            &client,
            ContainerAction::Describe {
                id: "run-container-id".into()
            }
        )
        .await
        .is_ok());
    }

    #[tokio::test]
    async fn test_describe_running_container_stats_unavailable() {
        let server = MockServer::start().await;
        let body = json!({
            "id": "no-stats-id",
            "names": ["no-stats-app"],
            "state": {
                "Status": "running",
                "Running": "true",
                "StartedAt": "2026-04-23T10:00:00+00:00",
                "Pid": "999"
            },
            "stats": {"Status": "StatsUnavailable"}
        });
        Mock::given(method("GET"))
            .and(path("/api/v1/containers/no-stats-id"))
            .respond_with(ResponseTemplate::new(200).set_body_json(body))
            .mount(&server)
            .await;
        let client = make_client(&server.uri()).await;
        assert!(handle(
            &client,
            ContainerAction::Describe {
                id: "no-stats-id".into()
            }
        )
        .await
        .is_ok());
    }

    #[tokio::test]
    async fn test_describe_running_container_no_stats_field() {
        // Running container but no "stats" key at all
        let server = MockServer::start().await;
        let body = json!({
            "id": "running-no-stat",
            "names": ["app"],
            "state": {"Status": "running", "Running": "true", "StartedAt": "2026-04-23T10:00:00+00:00", "Pid": "42"}
        });
        Mock::given(method("GET"))
            .and(path("/api/v1/containers/running-no-stat"))
            .respond_with(ResponseTemplate::new(200).set_body_json(body))
            .mount(&server)
            .await;
        let client = make_client(&server.uri()).await;
        assert!(handle(
            &client,
            ContainerAction::Describe {
                id: "running-no-stat".into()
            }
        )
        .await
        .is_ok());
    }

    #[tokio::test]
    async fn test_describe_exited_container_success() {
        let server = MockServer::start().await;
        let body = json!({
            "id": "exited-container",
            "names": ["finished-app"],
            "image": "alpine:3.18",
            "state": {
                "Status": "exited",
                "Running": "false",
                "ExitCode": "0",
                "StartedAt": "2026-04-23T09:00:00+00:00",
                "FinishedAt": "2026-04-23T09:00:05+00:00",
                "OOMKilled": "false"
            }
        });
        Mock::given(method("GET"))
            .and(path("/api/v1/containers/exited-container"))
            .respond_with(ResponseTemplate::new(200).set_body_json(body))
            .mount(&server)
            .await;
        let client = make_client(&server.uri()).await;
        assert!(handle(
            &client,
            ContainerAction::Describe {
                id: "exited-container".into()
            }
        )
        .await
        .is_ok());
    }

    #[tokio::test]
    async fn test_describe_exited_container_nonzero_exit() {
        let server = MockServer::start().await;
        let body = json!({
            "id": "failed-container",
            "names": ["crashed-app"],
            "state": {
                "Status": "exited",
                "Running": "false",
                "ExitCode": "1",
                "StartedAt": "2026-04-23T09:00:00+00:00",
                "FinishedAt": "2026-04-23T09:00:01+00:00",
                "OOMKilled": "false"
            }
        });
        Mock::given(method("GET"))
            .and(path("/api/v1/containers/failed-container"))
            .respond_with(ResponseTemplate::new(200).set_body_json(body))
            .mount(&server)
            .await;
        let client = make_client(&server.uri()).await;
        assert!(handle(
            &client,
            ContainerAction::Describe {
                id: "failed-container".into()
            }
        )
        .await
        .is_ok());
    }

    #[tokio::test]
    async fn test_describe_container_minimal_fields() {
        // Only id field present — all other fields use defaults
        let server = MockServer::start().await;
        let body = json!({"id": "bare-id"});
        Mock::given(method("GET"))
            .and(path("/api/v1/containers/bare-id"))
            .respond_with(ResponseTemplate::new(200).set_body_json(body))
            .mount(&server)
            .await;
        let client = make_client(&server.uri()).await;
        assert!(handle(
            &client,
            ContainerAction::Describe {
                id: "bare-id".into()
            }
        )
        .await
        .is_ok());
    }

    #[tokio::test]
    async fn test_describe_container_server_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/containers/bad-id"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;
        let client = make_client(&server.uri()).await;
        assert!(handle(
            &client,
            ContainerAction::Describe {
                id: "bad-id".into()
            }
        )
        .await
        .is_err());
    }

    // ── raw_containers ────────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_raw_containers_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/containers"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!([])))
            .mount(&server)
            .await;
        let client = make_client(&server.uri()).await;
        assert!(handle(&client, ContainerAction::Raw).await.is_ok());
    }

    #[tokio::test]
    async fn test_raw_containers_server_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/containers"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;
        let client = make_client(&server.uri()).await;
        assert!(handle(&client, ContainerAction::Raw).await.is_err());
    }

    #[tokio::test]
    async fn test_describe_running_container_zero_memory_limit() {
        // MemoryLimit == "0" → exercises the `else { 0.0 }` branch on line 283
        let server = MockServer::start().await;
        let body = json!({
            "id": "zero-mem-limit-id",
            "names": ["zero-mem-app"],
            "state": {
                "Status": "running",
                "Running": "true",
                "StartedAt": "2026-04-23T10:00:00+00:00",
                "Pid": "777"
            },
            "stats": {
                "CpuTotalUsage": "1000000000",
                "CpuUsageInKernelMode": "500000000",
                "CpuUsageInUserMode": "500000000",
                "MemoryUsage": "104857600",
                "MemoryLimit": "0",
                "Networks": "network: {rx_bytes: 0, tx_bytes: 0}"
            }
        });
        Mock::given(method("GET"))
            .and(path("/api/v1/containers/zero-mem-limit-id"))
            .respond_with(ResponseTemplate::new(200).set_body_json(body))
            .mount(&server)
            .await;
        let client = make_client(&server.uri()).await;
        assert!(handle(
            &client,
            ContainerAction::Describe {
                id: "zero-mem-limit-id".into()
            }
        )
        .await
        .is_ok());
    }
}
