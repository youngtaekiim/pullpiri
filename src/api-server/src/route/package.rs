// SPDX-License-Identifier: Apache-2.0

use axum::{
    extract::Path,
    response::IntoResponse,
    routing::{delete, get, post},
    Json, Router,
};

pub fn get_route() -> Router {
    Router::new()
        .route("/package", get(list_package))
        .route("/package/:name", get(inspect_package))
        .route("/package", post(import_package))
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
    super::status_ok()
}

async fn import_package(body: String) -> impl IntoResponse {
    println!("POST : package {body} is called.");
    let package = importer::parse_package(&body).await;

    if write_package_info_to_etcd(package.unwrap()).await.is_ok() {
        super::status_ok()
    } else {
        println!("error: writing package in etcd");
        super::status_err()
    }
}

async fn delete_package(Path(name): Path<String>) -> impl IntoResponse {
    // TODO
    println!("todo - delete {name}");
    super::status_ok()
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
