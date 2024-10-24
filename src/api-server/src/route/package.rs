// SPDX-License-Identifier: Apache-2.0

use axum::{
    body::Body,
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{delete, get, post},
    Json, Router,
};

pub fn get_route() -> Router {
    Router::new()
        .route("/package/", get(list_package))
        .route("/package/:name", get(inspect_package))
        .route("/package/:name", post(import_package))
        .route("/package/:name", delete(delete_package))
}

async fn list_package() -> Json<Vec<String>> {
    // TODO
    let packages = vec![String::new(), String::new()];
    Json(packages)
}

async fn inspect_package(Path(name): Path<String>) -> impl IntoResponse {
    // TODO
    println!("todo - inspect {name}");
    return_ok()
}

async fn import_package(Path(name): Path<String>, body: String) -> impl IntoResponse {
    let package = importer::parse_package(&body).await;

    println!("POST : package {name} is called.");
    println!("       Path is {body}.\n");

    if write_package_info_to_etcd(package.unwrap()).await.is_ok() {
        return_ok()
    } else {
        println!("error: writing package in etcd");
        return_err()
    }
}

async fn delete_package(Path(name): Path<String>) -> impl IntoResponse {
    // TODO
    println!("todo - delete {name}");
    return_ok()
}

fn return_ok() -> Response<Body> {
    Response::builder()
        .status(StatusCode::OK)
        .body(Body::from("Ok".to_string()))
        .unwrap()
}

fn return_err() -> Response<Body> {
    Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body(Body::from("Error".to_string()))
        .unwrap()
}

use importer::parser::package::Package;

async fn write_package_info_to_etcd(p: Package) -> Result<(), Box<dyn std::error::Error>> {
    let key_origin = format!("package/{}", p.name);

    for i in 0..p.model_names.len() {
        common::etcd::put(
            &format!("{key_origin}/models/{}", p.model_names.get(i).unwrap()),
            p.models.get(i).unwrap(),
        )
        .await?;
        common::etcd::put(
            &format!("{key_origin}/nodes/{}", p.model_names.get(i).unwrap()),
            p.nodes.get(i).unwrap(),
        )
        .await?;
        common::etcd::put(
            &format!("{key_origin}/networks/{}", p.model_names.get(i).unwrap()),
            p.networks.get(i).unwrap(),
        )
        .await?;
        common::etcd::put(
            &format!("{key_origin}/volumes/{}", p.model_names.get(i).unwrap()),
            p.volumes.get(i).unwrap(),
        )
        .await?;
    }

    Ok(())
}
