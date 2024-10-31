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
        .route("/package", post(handle_post))
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

async fn handle_post(body: String) -> impl IntoResponse {
    println!("POST : package {body} is called.\n");
    let result = import_package(body).await;

    if let Err(msg) = result {
        super::status_err(&msg.to_string())
    } else {
        super::status_ok()
    }
}

async fn import_package(body: String) -> Result<(), Box<dyn std::error::Error>> {
    let package = importer::parse_package(&body).await?;
    write_package_info_to_etcd(package).await?;

    Ok(())
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
        let model_name = p.model_names.get(i).unwrap();
        common::etcd::put(
            &format!("{key_origin}/models/{}", model_name),
            p.models.get(i).unwrap(),
        )
        .await?;
        common::etcd::put(
            &format!("{key_origin}/nodes/{}", model_name),
            p.nodes.get(i).unwrap(),
        )
        .await?;
        common::etcd::put(
            &format!("{key_origin}/networks/{}", model_name),
            p.networks.get(i).unwrap(),
        )
        .await?;
        common::etcd::put(
            &format!("{key_origin}/volumes/{}", model_name),
            p.volumes.get(i).unwrap(),
        )
        .await?;
    }

    Ok(())
}
