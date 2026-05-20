/*
* SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
* SPDX-License-Identifier: Apache-2.0
*/
//! REST API client for SettingsService

use crate::error::{CliError, Result};
use reqwest::Client;
use serde_json::Value;
use std::time::Duration;

/// HTTP client for communicating with SettingsService
pub struct SettingsClient {
    client: Client,
    base_url: String,
}

impl SettingsClient {
    /// Create a new SettingsClient
    ///
    /// # Arguments
    /// * `base_url` - Base URL of the SettingsService (e.g., "http://localhost:47098")
    /// * `timeout` - Request timeout in seconds
    pub fn new(base_url: &str, timeout: u64) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(timeout))
            .build()
            .map_err(CliError::Http)?;

        Ok(Self {
            client,
            base_url: base_url.trim_end_matches('/').to_string(),
        })
    }

    /// Make a GET request to the specified endpoint
    ///
    /// # Arguments
    /// * `endpoint` - API endpoint (e.g., "/api/v1/metrics")
    pub async fn get(&self, endpoint: &str) -> Result<Value> {
        let url = format!("{}{}", self.base_url, endpoint);
        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            return Err(CliError::Custom(format!(
                "Request failed with status: {}",
                response.status()
            )));
        }

        let bytes = response.bytes().await?;
        let json = if bytes.is_empty() {
            Value::Null
        } else {
            serde_json::from_slice(&bytes)?
        };
        Ok(json)
    }

    /// Make a POST request to the specified endpoint
    ///
    /// # Arguments
    /// * `endpoint` - API endpoint
    /// * `body` - Request body as JSON
    pub async fn post(&self, endpoint: &str, body: &Value) -> Result<Value> {
        let url = format!("{}{}", self.base_url, endpoint);
        let response = self.client.post(&url).json(body).send().await?;

        if !response.status().is_success() {
            return Err(CliError::Custom(format!(
                "Request failed with status: {}",
                response.status()
            )));
        }

        let json: Value = response.json().await?;
        Ok(json)
    }

    /// Make a PUT request to the specified endpoint
    ///
    /// # Arguments
    /// * `endpoint` - API endpoint
    /// * `body` - Request body as JSON
    pub async fn put(&self, endpoint: &str, body: &Value) -> Result<Value> {
        let url = format!("{}{}", self.base_url, endpoint);
        let response = self.client.put(&url).json(body).send().await?;

        if !response.status().is_success() {
            return Err(CliError::Custom(format!(
                "Request failed with status: {}",
                response.status()
            )));
        }

        let json: Value = response.json().await?;
        Ok(json)
    }

    /// Make a DELETE request to the specified endpoint
    ///
    /// # Arguments
    /// * `endpoint` - API endpoint
    pub async fn delete(&self, endpoint: &str) -> Result<Value> {
        let url = format!("{}{}", self.base_url, endpoint);
        let response = self.client.delete(&url).send().await?;

        if !response.status().is_success() {
            return Err(CliError::Custom(format!(
                "Request failed with status: {}",
                response.status()
            )));
        }

        let json: Value = response.json().await?;
        Ok(json)
    }

    /// Check if the SettingsService is reachable
    pub async fn health_check(&self) -> Result<bool> {
        match self.get("/api/v1/system/health").await {
            Ok(_) => Ok(true),
            Err(_) => {
                // Try alternative health check endpoint
                match self.get("/api/v1/health").await {
                    Ok(_) => Ok(true),
                    Err(_) => Ok(false),
                }
            }
        }
    }

    /// Apply YAML artifact (POST with text/plain content)
    ///
    /// # Arguments
    /// * `endpoint` - API endpoint (e.g., "/api/artifact")
    /// * `yaml_content` - YAML content as string
    pub async fn post_yaml(&self, endpoint: &str, yaml_content: &str) -> Result<Value> {
        let url = format!("{}{}", self.base_url, endpoint);
        let response = self
            .client
            .post(&url)
            .header("Content-Type", "text/plain")
            .body(yaml_content.to_owned())
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(CliError::Custom(format!(
                "Request failed with status: {} - {}",
                status, error_text
            )));
        }

        let json: Value = response.json().await?;
        Ok(json)
    }

    /// Withdraw YAML artifact (DELETE with text/plain content)
    ///
    /// # Arguments
    /// * `endpoint` - API endpoint (e.g., "/api/artifact")
    /// * `yaml_content` - YAML content as string
    pub async fn delete_yaml(&self, endpoint: &str, yaml_content: &str) -> Result<Value> {
        let url = format!("{}{}", self.base_url, endpoint);
        let response = self
            .client
            .delete(&url)
            .header("Content-Type", "text/plain")
            .body(yaml_content.to_owned())
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(CliError::Custom(format!(
                "Request failed with status: {} - {}",
                status, error_text
            )));
        }

        let json: Value = response.json().await?;
        Ok(json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::matchers::{body_string_contains, header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    // ── SettingsClient::new ───────────────────────────────────────────────────

    #[test]
    fn test_new_valid_url() {
        let client = SettingsClient::new("http://localhost:47098", 10);
        assert!(client.is_ok());
    }

    #[test]
    fn test_new_strips_trailing_slash() {
        let client = SettingsClient::new("http://localhost:47098/", 5);
        assert!(client.is_ok());
    }

    #[test]
    fn test_new_zero_timeout() {
        let client = SettingsClient::new("http://localhost:1234", 0);
        assert!(client.is_ok());
    }

    #[test]
    fn test_new_with_path_prefix() {
        let client = SettingsClient::new("http://10.0.0.1:8080", 30);
        assert!(client.is_ok());
    }

    // ── GET ───────────────────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_get_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/test"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"status": "ok"})))
            .mount(&server)
            .await;
        let client = SettingsClient::new(&server.uri(), 5).unwrap();
        let result = client.get("/api/v1/test").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap()["status"], "ok");
    }

    #[tokio::test]
    async fn test_get_empty_body() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/empty"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&server)
            .await;
        let client = SettingsClient::new(&server.uri(), 5).unwrap();
        let result = client.get("/api/v1/empty").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Value::Null);
    }

    #[tokio::test]
    async fn test_get_not_found() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/missing"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;
        let client = SettingsClient::new(&server.uri(), 5).unwrap();
        let result = client.get("/api/v1/missing").await;
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("404"));
    }

    #[tokio::test]
    async fn test_get_server_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/broken"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;
        let client = SettingsClient::new(&server.uri(), 5).unwrap();
        assert!(client.get("/api/v1/broken").await.is_err());
    }

    // ── POST ──────────────────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_post_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/create"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"created": true})))
            .mount(&server)
            .await;
        let client = SettingsClient::new(&server.uri(), 5).unwrap();
        let body = json!({"name": "test"});
        let result = client.post("/api/v1/create", &body).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap()["created"], true);
    }

    #[tokio::test]
    async fn test_post_server_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/create"))
            .respond_with(ResponseTemplate::new(400))
            .mount(&server)
            .await;
        let client = SettingsClient::new(&server.uri(), 5).unwrap();
        assert!(client.post("/api/v1/create", &json!({})).await.is_err());
    }

    // ── PUT ───────────────────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_put_success() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/api/v1/update"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"updated": true})))
            .mount(&server)
            .await;
        let client = SettingsClient::new(&server.uri(), 5).unwrap();
        let result = client.put("/api/v1/update", &json!({"key": "val"})).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_put_server_error() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/api/v1/update"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;
        let client = SettingsClient::new(&server.uri(), 5).unwrap();
        assert!(client.put("/api/v1/update", &json!({})).await.is_err());
    }

    // ── DELETE ────────────────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_delete_success() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/resource"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"deleted": true})))
            .mount(&server)
            .await;
        let client = SettingsClient::new(&server.uri(), 5).unwrap();
        let result = client.delete("/api/v1/resource").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_server_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/resource"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;
        let client = SettingsClient::new(&server.uri(), 5).unwrap();
        assert!(client.delete("/api/v1/resource").await.is_err());
    }

    // ── health_check ──────────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_health_check_primary_endpoint_ok() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/system/health"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"healthy": true})))
            .mount(&server)
            .await;
        let client = SettingsClient::new(&server.uri(), 5).unwrap();
        let result = client.health_check().await;
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[tokio::test]
    async fn test_health_check_fallback_endpoint() {
        // Primary fails → fallback succeeds
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/system/health"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/api/v1/health"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"ok": true})))
            .mount(&server)
            .await;
        let client = SettingsClient::new(&server.uri(), 5).unwrap();
        let result = client.health_check().await;
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[tokio::test]
    async fn test_health_check_both_fail_returns_false() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/system/health"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/api/v1/health"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;
        let client = SettingsClient::new(&server.uri(), 5).unwrap();
        let result = client.health_check().await;
        assert!(result.is_ok());
        assert!(!result.unwrap()); // returns false, not an error
    }

    // ── post_yaml ─────────────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_post_yaml_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/artifact"))
            .and(header("Content-Type", "text/plain"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!({"message": "Applied", "applied": []})),
            )
            .mount(&server)
            .await;
        let client = SettingsClient::new(&server.uri(), 5).unwrap();
        let yaml = "---\nkind: Scenario\nmetadata:\n  name: test\n";
        let result = client.post_yaml("/api/artifact", yaml).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_post_yaml_error_response() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/artifact"))
            .respond_with(ResponseTemplate::new(400).set_body_string("Bad Request"))
            .mount(&server)
            .await;
        let client = SettingsClient::new(&server.uri(), 5).unwrap();
        let result = client.post_yaml("/api/artifact", "invalid yaml").await;
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("400"));
    }

    // ── delete_yaml ───────────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_delete_yaml_success() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/artifact"))
            .and(header("Content-Type", "text/plain"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!({"message": "Withdrawn", "withdrawn": []})),
            )
            .mount(&server)
            .await;
        let client = SettingsClient::new(&server.uri(), 5).unwrap();
        let yaml = "---\nkind: Scenario\nmetadata:\n  name: test\n";
        let result = client.delete_yaml("/api/artifact", yaml).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_yaml_error_response() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/artifact"))
            .respond_with(ResponseTemplate::new(404).set_body_string("Not Found"))
            .mount(&server)
            .await;
        let client = SettingsClient::new(&server.uri(), 5).unwrap();
        let result = client.delete_yaml("/api/artifact", "some yaml").await;
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("404"));
    }

    #[tokio::test]
    async fn test_post_yaml_with_body_content() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/artifact"))
            .and(body_string_contains("Scenario"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(json!({"message": "ok", "applied": []})),
            )
            .mount(&server)
            .await;
        let client = SettingsClient::new(&server.uri(), 5).unwrap();
        let yaml = "kind: Scenario\nname: hello";
        assert!(client.post_yaml("/api/artifact", yaml).await.is_ok());
    }
}
