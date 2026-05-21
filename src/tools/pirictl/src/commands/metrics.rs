/*
* SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
* SPDX-License-Identifier: Apache-2.0
*/
//! Metrics command implementation

use crate::commands::{print_error, print_json, print_success};
use crate::{Result, SettingsClient};
use clap::Subcommand;

#[derive(Subcommand)]
pub enum MetricsAction {
    /// Get metrics in raw JSON format
    Raw,
}

/// Handle metrics commands
pub async fn handle(client: &SettingsClient, action: MetricsAction) -> Result<()> {
    match action {
        MetricsAction::Raw => get_metrics_raw(client).await,
    }
}

/// Get and display raw JSON metrics
async fn get_metrics_raw(client: &SettingsClient) -> Result<()> {
    print_success("Fetching raw metrics...");

    match client.get("/api/v1/metrics").await {
        Ok(metrics) => {
            print_json(&metrics)?;
            print_success("Raw metrics retrieved successfully");
        }
        Err(e) => {
            print_error(&format!("Failed to fetch raw metrics: {}", e));
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

    #[tokio::test]
    async fn test_handle_raw_success() {
        let server = MockServer::start().await;
        let body = json!({
            "cpu_usage": 42.5,
            "memory_total": 8589934592u64,
            "memory_used": 4294967296u64
        });
        Mock::given(method("GET"))
            .and(path("/api/v1/metrics"))
            .respond_with(ResponseTemplate::new(200).set_body_json(body))
            .mount(&server)
            .await;
        let client = make_client(&server.uri()).await;
        let result = handle(&client, MetricsAction::Raw).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_raw_server_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/metrics"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;
        let client = make_client(&server.uri()).await;
        let result = handle(&client, MetricsAction::Raw).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_handle_raw_empty_response() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/metrics"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
            .mount(&server)
            .await;
        let client = make_client(&server.uri()).await;
        assert!(handle(&client, MetricsAction::Raw).await.is_ok());
    }

    #[tokio::test]
    async fn test_handle_raw_array_response() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/metrics"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(json!([{"node": "n1"}, {"node": "n2"}])),
            )
            .mount(&server)
            .await;
        let client = make_client(&server.uri()).await;
        assert!(handle(&client, MetricsAction::Raw).await.is_ok());
    }
}
