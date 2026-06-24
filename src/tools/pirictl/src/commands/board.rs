/*
* SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
* SPDX-License-Identifier: Apache-2.0
*/
//! Board command implementation

use crate::commands::format::{format_bytes, format_duration_ago, format_memory};
use crate::commands::{print_error, print_info, print_json, print_success, print_table_header};
use crate::{Result, SettingsClient};
use clap::Subcommand;
use colored::Colorize;

#[derive(Subcommand)]
pub enum BoardAction {
    /// Get all boards
    Get,
    /// Describe specific board by ID
    Describe {
        /// Board ID
        id: String,
    },
    /// Get board information in raw JSON format
    Raw {
        /// Board ID (optional)
        id: Option<String>,
    },
}

/// Handle board commands
pub async fn handle(client: &SettingsClient, action: BoardAction) -> Result<()> {
    match action {
        BoardAction::Get => get_boards(client).await,
        BoardAction::Describe { id } => describe_board(client, &id).await,
        BoardAction::Raw { id } => {
            if let Some(board_id) = id {
                get_board_raw(client, &board_id).await
            } else {
                get_boards_raw(client).await
            }
        }
    }
}

/// Get all boards
async fn get_boards(client: &SettingsClient) -> Result<()> {
    print_info("Fetching boards list...");

    match client.get("/api/v1/boards").await {
        Ok(boards) => {
            print_table_header("Boards", &[("ID", 24), ("NODES", 10), ("SOCS", 10)]);

            // Look for "boards" array in the response
            if let Some(boards_array) = boards.get("boards").and_then(|b| b.as_array()) {
                if boards_array.is_empty() {
                    println!("No boards found.");
                } else {
                    // Print each board
                    for board in boards_array.iter() {
                        let id = board
                            .get("board_id")
                            .and_then(|i| i.as_str())
                            .unwrap_or("Unknown");
                        let node_count = board
                            .get("nodes")
                            .and_then(|n| n.as_array())
                            .map(|arr| arr.len())
                            .unwrap_or(0);
                        let soc_count = board
                            .get("socs")
                            .and_then(|s| s.as_array())
                            .map(|arr| arr.len())
                            .unwrap_or(0);

                        println!("{:<24} {:<10} {:<10}", id, node_count, soc_count);
                    }
                }
            } else if let Some(id) = boards.get("board_id") {
                // Single board response
                let node_count = boards
                    .get("nodes")
                    .and_then(|n| n.as_array())
                    .map(|arr| arr.len())
                    .unwrap_or(0);
                let soc_count = boards
                    .get("socs")
                    .and_then(|s| s.as_array())
                    .map(|arr| arr.len())
                    .unwrap_or(0);
                println!(
                    "{:<24} {:<10} {:<10}",
                    id.as_str().unwrap_or("Unknown"),
                    node_count,
                    soc_count
                );
            } else {
                println!("No boards found.");
            }

            println!();
            print_success("Boards list retrieved successfully");
        }
        Err(e) => {
            print_error(&format!("Failed to fetch boards: {}", e));
            return Err(e);
        }
    }

    Ok(())
}

/// Describe specific board information
async fn describe_board(client: &SettingsClient, board_id: &str) -> Result<()> {
    print_info(&format!("Fetching board information for ID: {}", board_id));

    let endpoint = format!("/api/v1/boards/{}", board_id);
    match client.get(&endpoint).await {
        Ok(board) => {
            // Board name
            let board_name = board
                .get("board_id")
                .and_then(|id| id.as_str())
                .unwrap_or(board_id);
            println!("\n{:<24}{}", format!("{}:", "Name".bold()), board_name);

            // Aggregated Resources
            println!("{}", "Aggregated Resources:".bold());

            if let (Some(cpu_count), Some(cpu_usage)) = (
                board.get("total_cpu_count").and_then(|c| c.as_u64()),
                board.get("total_cpu_usage").and_then(|u| u.as_f64()),
            ) {
                println!("  {:<22}{} ({:.2}% used)", "cpu:", cpu_count, cpu_usage);
            }

            if let Some(gpu_count) = board.get("total_gpu_count").and_then(|g| g.as_u64()) {
                println!("  {:<22}{}", "gpu:", gpu_count);
            }

            if let (Some(_total_mem), Some(used_mem), Some(mem_usage)) = (
                board.get("total_memory").and_then(|m| m.as_u64()),
                board.get("total_used_memory").and_then(|m| m.as_u64()),
                board.get("total_mem_usage").and_then(|u| u.as_f64()),
            ) {
                println!(
                    "  {:<22}{} ({:.2}% used)",
                    "memory:",
                    format_memory(used_mem),
                    mem_usage
                );
            }

            // Network I/O
            println!("{}", "Network I/O:".bold());
            if let Some(rx_bytes) = board.get("total_rx_bytes").and_then(|r| r.as_u64()) {
                println!("  {:<22}{}", "RX:", format_bytes(rx_bytes));
            }
            if let Some(tx_bytes) = board.get("total_tx_bytes").and_then(|t| t.as_u64()) {
                println!("  {:<22}{}", "TX:", format_bytes(tx_bytes));
            }

            // Disk I/O
            println!("{}", "Disk I/O:".bold());
            if let Some(read_bytes) = board.get("total_read_bytes").and_then(|r| r.as_u64()) {
                println!("  {:<22}{}", "Read:", format_bytes(read_bytes));
            }
            if let Some(write_bytes) = board.get("total_write_bytes").and_then(|w| w.as_u64()) {
                println!("  {:<22}{}", "Write:", format_bytes(write_bytes));
            }

            // SoCs
            if let Some(socs) = board.get("socs").and_then(|s| s.as_array()) {
                let soc_count = socs.len();
                println!("{} ({})", "SoCs:".bold(), soc_count);
                for soc in socs.iter() {
                    let soc_id = soc
                        .get("soc_id")
                        .and_then(|id| id.as_str())
                        .unwrap_or("Unknown");
                    let node_count = soc
                        .get("nodes")
                        .and_then(|n| n.as_array())
                        .map(|n| n.len())
                        .unwrap_or(0);
                    println!(
                        "  {:<22}({} node{})",
                        soc_id,
                        node_count,
                        if node_count == 1 { "" } else { "s" }
                    );
                }
            }

            // Nodes
            if let Some(nodes) = board.get("nodes").and_then(|n| n.as_array()) {
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
            if let Some(last_updated) = board.get("last_updated") {
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

            print_success("Board information retrieved successfully");
        }
        Err(e) => {
            print_error(&format!("Failed to fetch board {}: {}", board_id, e));
            return Err(e);
        }
    }

    Ok(())
}

/// Get boards in raw JSON format
async fn get_boards_raw(client: &SettingsClient) -> Result<()> {
    print_info("Fetching raw boards data...");

    match client.get("/api/v1/boards").await {
        Ok(boards) => {
            print_json(&boards)?;
            print_success("Raw boards data retrieved successfully");
        }
        Err(e) => {
            print_error(&format!("Failed to fetch raw boards data: {}", e));
            return Err(e);
        }
    }

    Ok(())
}

/// Get specific board in raw JSON format
async fn get_board_raw(client: &SettingsClient, board_id: &str) -> Result<()> {
    print_info(&format!("Fetching raw board data for ID: {}", board_id));

    let endpoint = format!("/api/v1/boards/{}", board_id);
    match client.get(&endpoint).await {
        Ok(board) => {
            print_json(&board)?;
            print_success("Raw board data retrieved successfully");
        }
        Err(e) => {
            print_error(&format!(
                "Failed to fetch raw board data for {}: {}",
                board_id, e
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
            .and(path("/api/v1/boards"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"boards": []})))
            .mount(&server)
            .await;
        let client = make_client(&server.uri()).await;
        let result = handle(&client, BoardAction::Get).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_describe() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/boards/board-1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"board_id": "board-1"})))
            .mount(&server)
            .await;
        let client = make_client(&server.uri()).await;
        let result = handle(
            &client,
            BoardAction::Describe {
                id: "board-1".into(),
            },
        )
        .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_raw_no_id() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/boards"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"boards": []})))
            .mount(&server)
            .await;
        let client = make_client(&server.uri()).await;
        let result = handle(&client, BoardAction::Raw { id: None }).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_raw_with_id() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/boards/b1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"board_id": "b1"})))
            .mount(&server)
            .await;
        let client = make_client(&server.uri()).await;
        let result = handle(
            &client,
            BoardAction::Raw {
                id: Some("b1".into()),
            },
        )
        .await;
        assert!(result.is_ok());
    }

    // ── get_boards ───────────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_get_boards_with_boards_array() {
        let server = MockServer::start().await;
        let body = json!({
            "boards": [
                {
                    "board_id": "board-alpha",
                    "nodes": [{"node_name": "n1"}, {"node_name": "n2"}],
                    "socs": [{"soc_id": "s1"}]
                }
            ]
        });
        Mock::given(method("GET"))
            .and(path("/api/v1/boards"))
            .respond_with(ResponseTemplate::new(200).set_body_json(body))
            .mount(&server)
            .await;
        let client = make_client(&server.uri()).await;
        let result = handle(&client, BoardAction::Get).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_boards_empty_array() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/boards"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"boards": []})))
            .mount(&server)
            .await;
        let client = make_client(&server.uri()).await;
        assert!(handle(&client, BoardAction::Get).await.is_ok());
    }

    #[tokio::test]
    async fn test_get_boards_single_board_response() {
        // Response is a single board object (no "boards" array key)
        let server = MockServer::start().await;
        let body = json!({
            "board_id": "solo-board",
            "nodes": [],
            "socs": []
        });
        Mock::given(method("GET"))
            .and(path("/api/v1/boards"))
            .respond_with(ResponseTemplate::new(200).set_body_json(body))
            .mount(&server)
            .await;
        let client = make_client(&server.uri()).await;
        assert!(handle(&client, BoardAction::Get).await.is_ok());
    }

    #[tokio::test]
    async fn test_get_boards_no_boards_key() {
        // Response has neither "boards" array nor "board_id"
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/boards"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
            .mount(&server)
            .await;
        let client = make_client(&server.uri()).await;
        assert!(handle(&client, BoardAction::Get).await.is_ok());
    }

    #[tokio::test]
    async fn test_get_boards_server_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/boards"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;
        let client = make_client(&server.uri()).await;
        assert!(handle(&client, BoardAction::Get).await.is_err());
    }

    // ── describe_board ───────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_describe_board_full_fields() {
        let server = MockServer::start().await;
        let body = json!({
            "board_id": "board-full",
            "total_cpu_count": 8,
            "total_cpu_usage": 42.5,
            "total_gpu_count": 2,
            "total_memory": 1073741824u64,
            "total_used_memory": 536870912u64,
            "total_mem_usage": 50.0,
            "total_rx_bytes": 1024u64,
            "total_tx_bytes": 2048u64,
            "total_read_bytes": 4096u64,
            "total_write_bytes": 8192u64,
            "socs": [{"soc_id": "soc-1", "nodes": [{"node_name": "n1"}]}],
            "nodes": [{"node_name": "node-1", "ip": "10.0.0.1"}],
            "last_updated": {"secs_since_epoch": 1000000000u64, "nanos_since_epoch": 0}
        });
        Mock::given(method("GET"))
            .and(path("/api/v1/boards/board-full"))
            .respond_with(ResponseTemplate::new(200).set_body_json(body))
            .mount(&server)
            .await;
        let client = make_client(&server.uri()).await;
        let result = handle(
            &client,
            BoardAction::Describe {
                id: "board-full".into(),
            },
        )
        .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_describe_board_missing_optional_fields() {
        let server = MockServer::start().await;
        // Minimal response - only board_id, no resource fields
        let body = json!({"board_id": "bare-board"});
        Mock::given(method("GET"))
            .and(path("/api/v1/boards/bare-board"))
            .respond_with(ResponseTemplate::new(200).set_body_json(body))
            .mount(&server)
            .await;
        let client = make_client(&server.uri()).await;
        assert!(handle(
            &client,
            BoardAction::Describe {
                id: "bare-board".into()
            }
        )
        .await
        .is_ok());
    }

    #[tokio::test]
    async fn test_describe_board_with_multiple_socs_and_nodes() {
        let server = MockServer::start().await;
        let body = json!({
            "board_id": "multi-board",
            "socs": [
                {"soc_id": "soc-a", "nodes": [{"node_name": "n1"}, {"node_name": "n2"}]},
                {"soc_id": "soc-b", "nodes": []}
            ],
            "nodes": [
                {"node_name": "node-a", "ip": "192.168.1.1"},
                {"node_name": "node-b", "ip": "192.168.1.2"}
            ]
        });
        Mock::given(method("GET"))
            .and(path("/api/v1/boards/multi-board"))
            .respond_with(ResponseTemplate::new(200).set_body_json(body))
            .mount(&server)
            .await;
        let client = make_client(&server.uri()).await;
        assert!(handle(
            &client,
            BoardAction::Describe {
                id: "multi-board".into()
            }
        )
        .await
        .is_ok());
    }

    #[tokio::test]
    async fn test_describe_board_server_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/boards/bad-id"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;
        let client = make_client(&server.uri()).await;
        assert!(handle(
            &client,
            BoardAction::Describe {
                id: "bad-id".into()
            }
        )
        .await
        .is_err());
    }

    // ── get_boards_raw / get_board_raw ───────────────────────────────────────

    #[tokio::test]
    async fn test_get_boards_raw_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/boards"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"boards": []})))
            .mount(&server)
            .await;
        let client = make_client(&server.uri()).await;
        assert!(handle(&client, BoardAction::Raw { id: None }).await.is_ok());
    }

    #[tokio::test]
    async fn test_get_boards_raw_server_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/boards"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;
        let client = make_client(&server.uri()).await;
        assert!(handle(&client, BoardAction::Raw { id: None })
            .await
            .is_err());
    }

    #[tokio::test]
    async fn test_get_board_raw_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/boards/b99"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"board_id": "b99"})))
            .mount(&server)
            .await;
        let client = make_client(&server.uri()).await;
        assert!(handle(
            &client,
            BoardAction::Raw {
                id: Some("b99".into())
            }
        )
        .await
        .is_ok());
    }

    #[tokio::test]
    async fn test_get_board_raw_server_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/boards/bad"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;
        let client = make_client(&server.uri()).await;
        assert!(handle(
            &client,
            BoardAction::Raw {
                id: Some("bad".into())
            }
        )
        .await
        .is_err());
    }
}
