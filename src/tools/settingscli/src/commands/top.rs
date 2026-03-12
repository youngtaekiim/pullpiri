/*
* SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
* SPDX-License-Identifier: Apache-2.0
*/
//! Top command implementation

use crate::commands::{print_error, print_success};
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
    print_success("Fetching system metrics...");

    match client.get("/api/v1/metrics").await {
        Ok(metrics) => {
            println!("\n{}", "System Metrics".bold());
            println!("{}", "=".repeat(50));

            // If the response is an array, iterate and print each metric
            if let Some(array) = metrics.as_array() {
                for (idx, metric) in array.iter().enumerate() {
                    println!("Metric #{}", idx + 1);
                    if let Some(component) = metric.get("component") {
                        println!("Component: {}", component.as_str().unwrap_or("Unknown"));
                    }
                    if let Some(metric_type) = metric.get("metric_type") {
                        println!("Metric Type: {}", metric_type.as_str().unwrap_or("Unknown"));
                    }
                    if let Some(timestamp) = metric.get("timestamp") {
                        println!("Timestamp: {}", timestamp.as_str().unwrap_or("Unknown"));
                    }
                    if let Some(value) = metric.get("value") {
                        if let Some(board_value) = value.get("value") {
                            if let Some(board_id) = board_value.get("board_id") {
                                println!("Board ID: {}", board_id.as_str().unwrap_or("Unknown"));
                            }
                            if let Some(nodes) = board_value.get("nodes").and_then(|n| n.as_array())
                            {
                                println!("\nNodes ({}):", nodes.len());
                                for (i, node) in nodes.iter().enumerate() {
                                    if let Some(node_name) = node.get("node_name") {
                                        println!(
                                            "  {}. {}",
                                            i + 1,
                                            node_name.as_str().unwrap_or("Unknown")
                                        );
                                        if let Some(cpu_usage) = node.get("cpu_usage") {
                                            println!(
                                                "     CPU Usage: {:.2}%",
                                                cpu_usage.as_f64().unwrap_or(0.0)
                                            );
                                        }
                                        if let Some(mem_usage) = node.get("mem_usage") {
                                            println!(
                                                "     Memory Usage: {:.2}%",
                                                mem_usage.as_f64().unwrap_or(0.0)
                                            );
                                        }
                                        if let Some(ip) = node.get("ip") {
                                            println!(
                                                "     IP: {}",
                                                ip.as_str().unwrap_or("Unknown")
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    }
                    println!("{}", "-".repeat(50));
                }
            } else {
                print_error("Metrics response is not an array.");
            }

            print_success("Metrics retrieved successfully");
        }
        Err(e) => {
            print_error(&format!("Failed to fetch metrics: {}", e));
            return Err(e);
        }
    }

    Ok(())
}
