/*
* SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
* SPDX-License-Identifier: Apache-2.0
*/
//! Top command implementation

use crate::commands::format::{format_bytes, format_memory};
use crate::commands::{print_error, print_info};
use crate::{Result, SettingsClient};
use clap::Subcommand;
use colored::Colorize;

#[derive(Subcommand)]
pub enum TopResource {
    /// Display live system metrics
    Metrics,
}

/// Handle top commands
pub async fn handle(client: &SettingsClient, resource: TopResource) -> Result<()> {
    match resource {
        TopResource::Metrics => top_metrics(client).await,
    }
}

/// Display and continuously update formatted metrics
async fn top_metrics(client: &SettingsClient) -> Result<()> {
    print_info("Fetching system metrics...");

    // Fetch all resource metrics
    let boards_result = client.get("/api/v1/metrics/boards").await;
    let socs_result = client.get("/api/v1/metrics/socs").await;
    let nodes_result = client.get("/api/v1/metrics/nodes").await;

    // Collect metrics data
    let mut metrics_data = Vec::new();

    // Process boards
    if let Ok(boards) = boards_result {
        if let Some(array) = boards.as_array() {
            for board in array {
                metrics_data.push(MetricRow::from_board(board));
            }
        }
    }

    // Process SoCs
    if let Ok(socs) = socs_result {
        if let Some(array) = socs.as_array() {
            for soc in array {
                metrics_data.push(MetricRow::from_soc(soc));
            }
        }
    }

    // Process nodes
    if let Ok(nodes) = nodes_result {
        if let Some(array) = nodes.as_array() {
            for node in array {
                metrics_data.push(MetricRow::from_node(node));
            }
        }
    }

    if metrics_data.is_empty() {
        print_error("No metrics data available");
        return Ok(());
    }

    // Print table header
    println!();
    println!(
        "{:<6} {:<15} {:<3} {:<5} {:<19} {:<5} {:<3} {:<7} {:<17} {:<21}",
        "LEVEL".bold(),
        "NAME/ID".bold(),
        "CPU".bold(),
        "CPU%".bold(),
        "MEMORY".bold(),
        "MEM%".bold(),
        "GPU".bold(),
        "ARCH".bold(),
        "NET(RX/TX)".bold(),
        "DISK(R/W)".bold()
    );

    // Print each metric row
    for metric in metrics_data {
        println!(
            "{:<6} {:<15} {:<3} {:<5} {:<19} {:<5} {:<3} {:<7} {:<17} {:<21}",
            metric.level,
            metric.name,
            metric.cpu,
            metric.cpu_percent,
            metric.memory,
            metric.mem_percent,
            metric.gpu,
            metric.arch,
            metric.network,
            metric.disk
        );
    }

    println!();
    Ok(())
}

/// Struct to hold a single metric row
struct MetricRow {
    level: String,
    name: String,
    cpu: String,
    cpu_percent: String,
    memory: String,
    mem_percent: String,
    gpu: String,
    arch: String,
    network: String,
    disk: String,
}

impl MetricRow {
    fn from_board(board: &serde_json::Value) -> Self {
        let board_id = board
            .get("board_id")
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown")
            .to_string();

        let cpu_count = board
            .get("total_cpu_count")
            .and_then(|v| v.as_u64())
            .unwrap_or(0);

        let cpu_usage = board
            .get("total_cpu_usage")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);

        let total_memory = board
            .get("total_memory")
            .and_then(|v| v.as_u64())
            .unwrap_or(0);

        let used_memory = board
            .get("total_used_memory")
            .and_then(|v| v.as_u64())
            .unwrap_or(0);

        let mem_usage = board
            .get("total_mem_usage")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);

        let gpu_count = board
            .get("total_gpu_count")
            .and_then(|v| v.as_u64())
            .unwrap_or(0);

        let network_rx = board
            .get("total_rx_bytes")
            .and_then(|v| v.as_u64())
            .unwrap_or(0);

        let network_tx = board
            .get("total_tx_bytes")
            .and_then(|v| v.as_u64())
            .unwrap_or(0);

        let disk_read = board
            .get("total_read_bytes")
            .and_then(|v| v.as_u64())
            .unwrap_or(0);

        let disk_write = board
            .get("total_write_bytes")
            .and_then(|v| v.as_u64())
            .unwrap_or(0);

        Self {
            level: "Board".to_string(),
            name: board_id,
            cpu: cpu_count.to_string(),
            cpu_percent: format!("{:.1}%", cpu_usage),
            memory: format!(
                "{} / {}",
                format_memory(used_memory),
                format_memory(total_memory)
            ),
            mem_percent: format!("{:.1}%", mem_usage),
            gpu: gpu_count.to_string(),
            arch: "-".to_string(),
            network: format!("{}/{}", format_bytes(network_rx), format_bytes(network_tx)),
            disk: format!("{}/{}", format_bytes(disk_read), format_bytes(disk_write)),
        }
    }

    fn from_soc(soc: &serde_json::Value) -> Self {
        let soc_id = soc
            .get("soc_id")
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown")
            .to_string();

        let cpu_count = soc
            .get("total_cpu_count")
            .and_then(|v| v.as_u64())
            .unwrap_or(0);

        let cpu_usage = soc
            .get("total_cpu_usage")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);

        let total_memory = soc
            .get("total_memory")
            .and_then(|v| v.as_u64())
            .unwrap_or(0);

        let used_memory = soc
            .get("total_used_memory")
            .and_then(|v| v.as_u64())
            .unwrap_or(0);

        let mem_usage = soc
            .get("total_mem_usage")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);

        let gpu_count = soc
            .get("total_gpu_count")
            .and_then(|v| v.as_u64())
            .unwrap_or(0);

        let network_rx = soc
            .get("total_rx_bytes")
            .and_then(|v| v.as_u64())
            .unwrap_or(0);

        let network_tx = soc
            .get("total_tx_bytes")
            .and_then(|v| v.as_u64())
            .unwrap_or(0);

        let disk_read = soc
            .get("total_read_bytes")
            .and_then(|v| v.as_u64())
            .unwrap_or(0);

        let disk_write = soc
            .get("total_write_bytes")
            .and_then(|v| v.as_u64())
            .unwrap_or(0);

        Self {
            level: "SoC".to_string(),
            name: soc_id,
            cpu: cpu_count.to_string(),
            cpu_percent: format!("{:.1}%", cpu_usage),
            memory: format!(
                "{} / {}",
                format_memory(used_memory),
                format_memory(total_memory)
            ),
            mem_percent: format!("{:.1}%", mem_usage),
            gpu: gpu_count.to_string(),
            arch: "-".to_string(),
            network: format!("{}/{}", format_bytes(network_rx), format_bytes(network_tx)),
            disk: format!("{}/{}", format_bytes(disk_read), format_bytes(disk_write)),
        }
    }

    fn from_node(node: &serde_json::Value) -> Self {
        let node_name = node
            .get("node_name")
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown")
            .to_string();

        let cpu_count = node.get("cpu_count").and_then(|v| v.as_u64()).unwrap_or(0);

        let cpu_usage = node
            .get("cpu_usage")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);

        let total_memory = node
            .get("total_memory")
            .and_then(|v| v.as_u64())
            .unwrap_or(0);

        let used_memory = node
            .get("used_memory")
            .and_then(|v| v.as_u64())
            .unwrap_or(0);

        let mem_usage = node
            .get("mem_usage")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);

        let gpu_count = node.get("gpu_count").and_then(|v| v.as_u64()).unwrap_or(0);

        let arch = node
            .get("arch")
            .and_then(|v| v.as_str())
            .unwrap_or("-")
            .to_string();

        let network_rx = node.get("rx_bytes").and_then(|v| v.as_u64()).unwrap_or(0);

        let network_tx = node.get("tx_bytes").and_then(|v| v.as_u64()).unwrap_or(0);

        let disk_read = node.get("read_bytes").and_then(|v| v.as_u64()).unwrap_or(0);

        let disk_write = node
            .get("write_bytes")
            .and_then(|v| v.as_u64())
            .unwrap_or(0);

        Self {
            level: "Node".to_string(),
            name: node_name,
            cpu: cpu_count.to_string(),
            cpu_percent: format!("{:.1}%", cpu_usage),
            memory: format!(
                "{} / {}",
                format_memory(used_memory),
                format_memory(total_memory)
            ),
            mem_percent: format!("{:.1}%", mem_usage),
            gpu: gpu_count.to_string(),
            arch,
            network: format!("{}/{}", format_bytes(network_rx), format_bytes(network_tx)),
            disk: format!("{}/{}", format_bytes(disk_read), format_bytes(disk_write)),
        }
    }
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

    // ── MetricRow::from_board ─────────────────────────────────────────────────

    #[test]
    fn test_metric_row_from_board_full_fields() {
        let board = json!({
            "board_id": "board-x",
            "total_cpu_count": 16,
            "total_cpu_usage": 75.5,
            "total_memory": 17179869184u64,
            "total_used_memory": 8589934592u64,
            "total_mem_usage": 50.0,
            "total_gpu_count": 4,
            "total_rx_bytes": 1048576u64,
            "total_tx_bytes": 2097152u64,
            "total_read_bytes": 4194304u64,
            "total_write_bytes": 8388608u64
        });
        let row = MetricRow::from_board(&board);
        assert_eq!(row.level, "Board");
        assert_eq!(row.name, "board-x");
        assert_eq!(row.cpu, "16");
        assert!(row.cpu_percent.contains("75.5"));
        assert_eq!(row.gpu, "4");
        assert_eq!(row.arch, "-");
    }

    #[test]
    fn test_metric_row_from_board_missing_fields() {
        let board = json!({});
        let row = MetricRow::from_board(&board);
        assert_eq!(row.level, "Board");
        assert_eq!(row.name, "Unknown");
        assert_eq!(row.cpu, "0");
        assert_eq!(row.gpu, "0");
    }

    #[test]
    fn test_metric_row_from_board_zero_values() {
        let board = json!({
            "board_id": "zero-board",
            "total_cpu_count": 0,
            "total_cpu_usage": 0.0,
            "total_memory": 0u64,
            "total_used_memory": 0u64,
            "total_mem_usage": 0.0,
            "total_gpu_count": 0,
            "total_rx_bytes": 0u64,
            "total_tx_bytes": 0u64,
            "total_read_bytes": 0u64,
            "total_write_bytes": 0u64
        });
        let row = MetricRow::from_board(&board);
        assert_eq!(row.name, "zero-board");
        assert!(row.cpu_percent.contains("0.0"));
    }

    // ── MetricRow::from_soc ───────────────────────────────────────────────────

    #[test]
    fn test_metric_row_from_soc_full_fields() {
        let soc = json!({
            "soc_id": "soc-y",
            "total_cpu_count": 8,
            "total_cpu_usage": 30.0,
            "total_memory": 8589934592u64,
            "total_used_memory": 2147483648u64,
            "total_mem_usage": 25.0,
            "total_gpu_count": 1,
            "total_rx_bytes": 512u64,
            "total_tx_bytes": 1024u64,
            "total_read_bytes": 2048u64,
            "total_write_bytes": 4096u64
        });
        let row = MetricRow::from_soc(&soc);
        assert_eq!(row.level, "SoC");
        assert_eq!(row.name, "soc-y");
        assert_eq!(row.cpu, "8");
        assert!(row.cpu_percent.contains("30.0"));
        assert_eq!(row.gpu, "1");
        assert_eq!(row.arch, "-");
    }

    #[test]
    fn test_metric_row_from_soc_missing_fields() {
        let soc = json!({});
        let row = MetricRow::from_soc(&soc);
        assert_eq!(row.level, "SoC");
        assert_eq!(row.name, "Unknown");
        assert_eq!(row.cpu, "0");
    }

    // ── MetricRow::from_node ──────────────────────────────────────────────────

    #[test]
    fn test_metric_row_from_node_full_fields() {
        let node = json!({
            "node_name": "node-z",
            "cpu_count": 4,
            "cpu_usage": 55.0,
            "total_memory": 4294967296u64,
            "used_memory": 2147483648u64,
            "mem_usage": 50.0,
            "gpu_count": 0,
            "arch": "arm64",
            "rx_bytes": 256u64,
            "tx_bytes": 512u64,
            "read_bytes": 1024u64,
            "write_bytes": 2048u64
        });
        let row = MetricRow::from_node(&node);
        assert_eq!(row.level, "Node");
        assert_eq!(row.name, "node-z");
        assert_eq!(row.cpu, "4");
        assert!(row.cpu_percent.contains("55.0"));
        assert_eq!(row.arch, "arm64");
    }

    #[test]
    fn test_metric_row_from_node_missing_fields() {
        let node = json!({});
        let row = MetricRow::from_node(&node);
        assert_eq!(row.level, "Node");
        assert_eq!(row.name, "Unknown");
        assert_eq!(row.arch, "-");
    }

    // ── handle() / top_metrics() ──────────────────────────────────────────────

    #[tokio::test]
    async fn test_handle_metrics_with_data() {
        let server = MockServer::start().await;

        let boards = json!([{
            "board_id": "b1",
            "total_cpu_count": 4,
            "total_cpu_usage": 10.0,
            "total_memory": 1073741824u64,
            "total_used_memory": 536870912u64,
            "total_mem_usage": 50.0,
            "total_gpu_count": 0,
            "total_rx_bytes": 0u64,
            "total_tx_bytes": 0u64,
            "total_read_bytes": 0u64,
            "total_write_bytes": 0u64
        }]);
        let socs = json!([{
            "soc_id": "s1",
            "total_cpu_count": 2,
            "total_cpu_usage": 5.0,
            "total_memory": 536870912u64,
            "total_used_memory": 268435456u64,
            "total_mem_usage": 50.0,
            "total_gpu_count": 0,
            "total_rx_bytes": 0u64,
            "total_tx_bytes": 0u64,
            "total_read_bytes": 0u64,
            "total_write_bytes": 0u64
        }]);
        let nodes = json!([{
            "node_name": "n1",
            "cpu_count": 2,
            "cpu_usage": 5.0,
            "total_memory": 268435456u64,
            "used_memory": 134217728u64,
            "mem_usage": 50.0,
            "gpu_count": 0,
            "arch": "amd64",
            "rx_bytes": 0u64,
            "tx_bytes": 0u64,
            "read_bytes": 0u64,
            "write_bytes": 0u64
        }]);

        Mock::given(method("GET"))
            .and(path("/api/v1/metrics/boards"))
            .respond_with(ResponseTemplate::new(200).set_body_json(boards))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/api/v1/metrics/socs"))
            .respond_with(ResponseTemplate::new(200).set_body_json(socs))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/api/v1/metrics/nodes"))
            .respond_with(ResponseTemplate::new(200).set_body_json(nodes))
            .mount(&server)
            .await;

        let client = make_client(&server.uri()).await;
        let result = handle(&client, TopResource::Metrics).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_metrics_all_endpoints_fail() {
        // All three API calls fail → empty metrics_data → prints error but returns Ok(())
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/metrics/boards"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/api/v1/metrics/socs"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/api/v1/metrics/nodes"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;

        let client = make_client(&server.uri()).await;
        let result = handle(&client, TopResource::Metrics).await;
        // Empty metrics → print_error is called but Ok(()) is returned
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_metrics_non_array_responses() {
        // API returns objects instead of arrays → no rows appended → empty path
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/metrics/boards"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/api/v1/metrics/socs"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/api/v1/metrics/nodes"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
            .mount(&server)
            .await;

        let client = make_client(&server.uri()).await;
        let result = handle(&client, TopResource::Metrics).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_metrics_empty_arrays() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/metrics/boards"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!([])))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/api/v1/metrics/socs"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!([])))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/api/v1/metrics/nodes"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!([])))
            .mount(&server)
            .await;

        let client = make_client(&server.uri()).await;
        let result = handle(&client, TopResource::Metrics).await;
        assert!(result.is_ok());
    }
}
