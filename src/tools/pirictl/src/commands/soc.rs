/*
* SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
* SPDX-License-Identifier: Apache-2.0
*/
//! SoC command implementation

use crate::commands::format::{format_bytes, format_duration_ago, format_memory};
use crate::commands::{print_error, print_info, print_json, print_success, print_table_header};
use crate::{Result, SettingsClient};
use clap::Subcommand;
use colored::Colorize;

#[derive(Subcommand)]
pub enum SocAction {
    /// Get all SoCs
    Get,
    /// Describe specific SoC by ID
    Describe {
        /// SoC ID
        id: String,
    },
    /// Get SoC information in raw JSON format
    Raw {
        /// SoC ID (optional)
        id: Option<String>,
    },
}

/// Handle SoC commands
pub async fn handle(client: &SettingsClient, action: SocAction) -> Result<()> {
    match action {
        SocAction::Get => get_socs(client).await,
        SocAction::Describe { id } => describe_soc(client, &id).await,
        SocAction::Raw { id } => {
            if let Some(soc_id) = id {
                get_soc_raw(client, &soc_id).await
            } else {
                get_socs_raw(client).await
            }
        }
    }
}

/// Get all SoCs
async fn get_socs(client: &SettingsClient) -> Result<()> {
    print_info("Fetching SoCs list...");

    match client.get("/api/v1/socs").await {
        Ok(socs) => {
            print_table_header("SoCs", &[("ID", 24), ("NODES", 10)]);

            // Look for "socs" array in the response
            if let Some(socs_array) = socs.get("socs").and_then(|s| s.as_array()) {
                if socs_array.is_empty() {
                    println!("No SoCs found.");
                } else {
                    // Print each SoC
                    for soc in socs_array.iter() {
                        let id = soc
                            .get("soc_id")
                            .and_then(|i| i.as_str())
                            .unwrap_or("Unknown");
                        let node_count = soc
                            .get("nodes")
                            .and_then(|n| n.as_array())
                            .map(|arr| arr.len())
                            .unwrap_or(0);

                        println!("{:<24} {:<10}", id, node_count);
                    }
                }
            } else if let Some(id) = socs.get("soc_id") {
                // Single SoC response
                let node_count = socs
                    .get("nodes")
                    .and_then(|n| n.as_array())
                    .map(|arr| arr.len())
                    .unwrap_or(0);
                println!(
                    "{:<24} {:<10}",
                    id.as_str().unwrap_or("Unknown"),
                    node_count
                );
            } else {
                println!("No SoCs found.");
            }

            println!();
            print_success("SoCs list retrieved successfully");
        }
        Err(e) => {
            print_error(&format!("Failed to fetch SoCs: {}", e));
            return Err(e);
        }
    }

    Ok(())
}

/// Describe specific SoC information
async fn describe_soc(client: &SettingsClient, soc_id: &str) -> Result<()> {
    print_info(&format!("Fetching SoC information for ID: {}", soc_id));

    let endpoint = format!("/api/v1/socs/{}", soc_id);
    match client.get(&endpoint).await {
        Ok(soc) => {
            // SoC name
            let soc_name = soc
                .get("soc_id")
                .and_then(|id| id.as_str())
                .unwrap_or(soc_id);
            println!("\n{:<24}{}", format!("{}:", "Name".bold()), soc_name);

            // Status (default to Active if not provided)
            let status = soc
                .get("status")
                .and_then(|s| s.as_str())
                .unwrap_or("Active");
            println!("{:<24}{}", format!("{}:", "Status".bold()), status);

            // Aggregated Resources
            println!("{}", "Aggregated Resources:".bold());

            if let (Some(cpu_count), Some(cpu_usage)) = (
                soc.get("total_cpu_count").and_then(|c| c.as_u64()),
                soc.get("total_cpu_usage").and_then(|u| u.as_f64()),
            ) {
                println!("  {:<22}{} ({:.2}% used)", "cpu:", cpu_count, cpu_usage);
            }

            if let Some(gpu_count) = soc.get("total_gpu_count").and_then(|g| g.as_u64()) {
                println!("  {:<22}{}", "gpu:", gpu_count);
            }

            if let (Some(used_memory), Some(mem_usage)) = (
                soc.get("total_used_memory").and_then(|m| m.as_u64()),
                soc.get("total_mem_usage").and_then(|u| u.as_f64()),
            ) {
                println!(
                    "  {:<22}{} ({:.2}% used)",
                    "memory:",
                    format_memory(used_memory),
                    mem_usage
                );
            }

            // Network I/O
            println!("{}", "Network I/O:".bold());
            if let Some(rx_bytes) = soc.get("total_rx_bytes").and_then(|r| r.as_u64()) {
                println!("  {:<22}{}", "RX:", format_bytes(rx_bytes));
            }
            if let Some(tx_bytes) = soc.get("total_tx_bytes").and_then(|t| t.as_u64()) {
                println!("  {:<22}{}", "TX:", format_bytes(tx_bytes));
            }

            // Disk I/O
            println!("{}", "Disk I/O:".bold());
            if let Some(read_bytes) = soc.get("total_read_bytes").and_then(|r| r.as_u64()) {
                println!("  {:<22}{}", "Read:", format_bytes(read_bytes));
            }
            if let Some(write_bytes) = soc.get("total_write_bytes").and_then(|w| w.as_u64()) {
                println!("  {:<22}{}", "Write:", format_bytes(write_bytes));
            }

            // Nodes
            if let Some(nodes) = soc.get("nodes").and_then(|n| n.as_array()) {
                let node_count = nodes.len();
                println!("{} ({})", "Nodes:".bold(), node_count);
                for node in nodes.iter() {
                    let node_name = node
                        .get("node_name")
                        .and_then(|n| n.as_str())
                        .unwrap_or("Unknown");
                    let node_ip = node.get("ip").and_then(|ip| ip.as_str()).unwrap_or("N/A");
                    println!("  {:<22}{}", node_name, node_ip);
                }
            }

            // Last Updated
            if let Some(last_updated) = soc.get("last_updated") {
                if let Some(secs) = last_updated
                    .get("secs_since_epoch")
                    .and_then(|s| s.as_u64())
                {
                    let now = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs();
                    let elapsed = now.saturating_sub(secs);
                    println!(
                        "{:<24}{}",
                        format!("{}:", "Last Updated".bold()),
                        format_duration_ago(elapsed)
                    );
                }
            }

            // Hint
            println!("\n{}", "For more details:".dimmed());
            println!("  {}", "pirictl describe node <node_name>".dimmed());

            print_success("SoC information retrieved successfully");
        }
        Err(e) => {
            print_error(&format!("Failed to fetch SoC {}: {}", soc_id, e));
            return Err(e);
        }
    }

    Ok(())
}

/// Get SoCs in raw JSON format
async fn get_socs_raw(client: &SettingsClient) -> Result<()> {
    print_info("Fetching raw SoCs data...");

    match client.get("/api/v1/socs").await {
        Ok(socs) => {
            print_json(&socs)?;
            print_success("Raw SoCs data retrieved successfully");
        }
        Err(e) => {
            print_error(&format!("Failed to fetch raw SoCs data: {}", e));
            return Err(e);
        }
    }

    Ok(())
}

/// Get specific SoC in raw JSON format
async fn get_soc_raw(client: &SettingsClient, soc_id: &str) -> Result<()> {
    print_info(&format!("Fetching raw SoC data for ID: {}", soc_id));

    let endpoint = format!("/api/v1/socs/{}", soc_id);
    match client.get(&endpoint).await {
        Ok(soc) => {
            print_json(&soc)?;
            print_success("Raw SoC data retrieved successfully");
        }
        Err(e) => {
            print_error(&format!(
                "Failed to fetch raw SoC data for {}: {}",
                soc_id, e
            ));
            return Err(e);
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

    // ── handle() dispatch ────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_handle_get() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/socs"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"socs": []})))
            .mount(&server)
            .await;
        let client = make_client(&server.uri()).await;
        assert!(handle(&client, SocAction::Get).await.is_ok());
    }

    #[tokio::test]
    async fn test_handle_describe() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/socs/soc-1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"soc_id": "soc-1"})))
            .mount(&server)
            .await;
        let client = make_client(&server.uri()).await;
        assert!(handle(&client, SocAction::Describe { id: "soc-1".into() })
            .await
            .is_ok());
    }

    #[tokio::test]
    async fn test_handle_raw_no_id() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/socs"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"socs": []})))
            .mount(&server)
            .await;
        let client = make_client(&server.uri()).await;
        assert!(handle(&client, SocAction::Raw { id: None }).await.is_ok());
    }

    #[tokio::test]
    async fn test_handle_raw_with_id() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/socs/s1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"soc_id": "s1"})))
            .mount(&server)
            .await;
        let client = make_client(&server.uri()).await;
        assert!(handle(
            &client,
            SocAction::Raw {
                id: Some("s1".into())
            }
        )
        .await
        .is_ok());
    }

    // ── get_socs ─────────────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_get_socs_with_array() {
        let server = MockServer::start().await;
        let body = json!({
            "socs": [
                {"soc_id": "soc-alpha", "nodes": [{"node_name": "n1"}, {"node_name": "n2"}]},
                {"soc_id": "soc-beta",  "nodes": []}
            ]
        });
        Mock::given(method("GET"))
            .and(path("/api/v1/socs"))
            .respond_with(ResponseTemplate::new(200).set_body_json(body))
            .mount(&server)
            .await;
        let client = make_client(&server.uri()).await;
        assert!(handle(&client, SocAction::Get).await.is_ok());
    }

    #[tokio::test]
    async fn test_get_socs_empty_array() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/socs"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"socs": []})))
            .mount(&server)
            .await;
        let client = make_client(&server.uri()).await;
        assert!(handle(&client, SocAction::Get).await.is_ok());
    }

    #[tokio::test]
    async fn test_get_socs_single_soc_response() {
        let server = MockServer::start().await;
        let body = json!({"soc_id": "solo-soc", "nodes": []});
        Mock::given(method("GET"))
            .and(path("/api/v1/socs"))
            .respond_with(ResponseTemplate::new(200).set_body_json(body))
            .mount(&server)
            .await;
        let client = make_client(&server.uri()).await;
        assert!(handle(&client, SocAction::Get).await.is_ok());
    }

    #[tokio::test]
    async fn test_get_socs_no_soc_key() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/socs"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
            .mount(&server)
            .await;
        let client = make_client(&server.uri()).await;
        assert!(handle(&client, SocAction::Get).await.is_ok());
    }

    #[tokio::test]
    async fn test_get_socs_server_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/socs"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;
        let client = make_client(&server.uri()).await;
        assert!(handle(&client, SocAction::Get).await.is_err());
    }

    // ── describe_soc ──────────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_describe_soc_full_fields() {
        let server = MockServer::start().await;
        let body = json!({
            "soc_id": "full-soc",
            "status": "Active",
            "total_cpu_count": 8,
            "total_cpu_usage": 60.0,
            "total_gpu_count": 2,
            "total_used_memory": 2147483648u64,
            "total_mem_usage": 25.0,
            "total_rx_bytes": 512u64,
            "total_tx_bytes": 1024u64,
            "total_read_bytes": 2048u64,
            "total_write_bytes": 4096u64,
            "nodes": [
                {"node_name": "node-x", "ip": "10.10.10.1"},
                {"node_name": "node-y", "ip": "10.10.10.2"}
            ],
            "last_updated": {"secs_since_epoch": 1000000000u64, "nanos_since_epoch": 0}
        });
        Mock::given(method("GET"))
            .and(path("/api/v1/socs/full-soc"))
            .respond_with(ResponseTemplate::new(200).set_body_json(body))
            .mount(&server)
            .await;
        let client = make_client(&server.uri()).await;
        assert!(handle(
            &client,
            SocAction::Describe {
                id: "full-soc".into()
            }
        )
        .await
        .is_ok());
    }

    #[tokio::test]
    async fn test_describe_soc_minimal_fields() {
        let server = MockServer::start().await;
        let body = json!({"soc_id": "bare-soc"});
        Mock::given(method("GET"))
            .and(path("/api/v1/socs/bare-soc"))
            .respond_with(ResponseTemplate::new(200).set_body_json(body))
            .mount(&server)
            .await;
        let client = make_client(&server.uri()).await;
        assert!(handle(
            &client,
            SocAction::Describe {
                id: "bare-soc".into()
            }
        )
        .await
        .is_ok());
    }

    #[tokio::test]
    async fn test_describe_soc_server_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/socs/bad-soc"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;
        let client = make_client(&server.uri()).await;
        assert!(handle(
            &client,
            SocAction::Describe {
                id: "bad-soc".into()
            }
        )
        .await
        .is_err());
    }

    // ── raw endpoints ─────────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_get_socs_raw_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/socs"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"socs": []})))
            .mount(&server)
            .await;
        let client = make_client(&server.uri()).await;
        assert!(handle(&client, SocAction::Raw { id: None }).await.is_ok());
    }

    #[tokio::test]
    async fn test_get_socs_raw_server_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/socs"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;
        let client = make_client(&server.uri()).await;
        assert!(handle(&client, SocAction::Raw { id: None }).await.is_err());
    }

    #[tokio::test]
    async fn test_get_soc_raw_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/socs/s99"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"soc_id": "s99"})))
            .mount(&server)
            .await;
        let client = make_client(&server.uri()).await;
        assert!(handle(
            &client,
            SocAction::Raw {
                id: Some("s99".into())
            }
        )
        .await
        .is_ok());
    }

    #[tokio::test]
    async fn test_get_soc_raw_server_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/socs/bad"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;
        let client = make_client(&server.uri()).await;
        assert!(handle(
            &client,
            SocAction::Raw {
                id: Some("bad".into())
            }
        )
        .await
        .is_err());
    }
}
