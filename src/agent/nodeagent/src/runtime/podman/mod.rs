/*
* SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
* SPDX-License-Identifier: Apache-2.0
*/

pub mod container;
pub mod resource;

use bytes::Bytes;
use common::nodeagent::fromactioncontroller::WorkloadCommand;
use http_body_util::{BodyExt, Full};
use hyper::{Method, Request, Uri};
use hyper_util::{client::legacy::Client, rt::TokioExecutor};
use hyperlocal::{UnixConnector, Uri as UnixUri};
use once_cell::sync::Lazy;

// Modify this if you want to run without root authorization
// or if you have a different socket path.
// For example, if you run Podman as root, you might use:
// "/var/run/podman/podman.sock"
// Or if you run it as a user, you might use:
// "/run/user/1000/podman/podman.sock"
//
// The default may be overridden at runtime with the `PODMAN_SOCKET`
// environment variable (useful for rootless Podman and for testing).
const DEFAULT_PODMAN_SOCKET: &str = "/var/run/podman/podman.sock";

/// Resolve the Podman socket path, honoring the `PODMAN_SOCKET` override.
static PODMAN_SOCKET: Lazy<String> = Lazy::new(|| {
    std::env::var("PODMAN_SOCKET").unwrap_or_else(|_| DEFAULT_PODMAN_SOCKET.to_string())
});

// A single `hyper::Client` is cheap to clone and manages its own connection
// pool internally, so it is created once and reused for every request
// instead of being rebuilt on each call to `get`/`post`/`delete`.
pub type PodmanBody = Full<Bytes>;
pub type PodmanResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

static PODMAN_CLIENT: Lazy<Client<UnixConnector, PodmanBody>> =
    Lazy::new(|| Client::builder(TokioExecutor::new()).build(UnixConnector));

/// Build an empty request body for Podman API calls.
pub fn empty_body() -> PodmanBody {
    Full::new(Bytes::new())
}

/// Build a request body from bytes for Podman API calls.
pub fn body_from<T: Into<Bytes>>(body: T) -> PodmanBody {
    Full::new(body.into())
}

pub async fn get(path: &str) -> PodmanResult<Bytes> {
    let uri: Uri = UnixUri::new(PODMAN_SOCKET.as_str(), path).into();

    let res = PODMAN_CLIENT.get(uri).await?;
    Ok(res.into_body().collect().await?.to_bytes())
}

pub async fn post(path: &str, body: PodmanBody) -> PodmanResult<Bytes> {
    let uri: Uri = UnixUri::new(PODMAN_SOCKET.as_str(), path).into();

    let req = Request::builder()
        .method(Method::POST)
        .uri(uri)
        .body(body)?;

    let res = PODMAN_CLIENT.request(req).await?;
    Ok(res.into_body().collect().await?.to_bytes())
}

pub async fn delete(path: &str) -> PodmanResult<Bytes> {
    let uri: Uri = UnixUri::new(PODMAN_SOCKET.as_str(), path).into();

    let req = Request::builder()
        .method(Method::DELETE)
        .uri(uri)
        .body(empty_body())?;

    let res = PODMAN_CLIENT.request(req).await?;
    Ok(res.into_body().collect().await?.to_bytes())
}

pub async fn handle_workload(
    command: i32,
    pod: &str,
) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
    println!(
        "handle_workload called with command: {} for model(pod)",
        command
    );
    match command {
        x if x == WorkloadCommand::Start as i32 => {
            let container_ids = container::start(pod).await?;
            return Ok(container_ids);
        }
        x if x == WorkloadCommand::Stop as i32 => {
            container::stop(pod).await?;
        }
        x if x == WorkloadCommand::Restart as i32 => {
            container::restart(pod).await?;
        }
        _ => {
            // Do nothing for unimplemented commands
            return Err("unimplemented command".into());
        }
    };

    Ok(vec![])
}

//Unit tets cases
#[cfg(test)]
mod tests {
    use super::get;
    use bytes::Bytes;
    use tokio;

    #[tokio::test]
    async fn test_get_with_valid_path() {
        let result: Result<Bytes, Box<dyn std::error::Error + Send + Sync>> =
            get("/v1.0/version").await;
        assert!(result.is_ok());
        if let Ok(bytes) = result {
            assert!(!bytes.is_empty());
        }
    }
}
