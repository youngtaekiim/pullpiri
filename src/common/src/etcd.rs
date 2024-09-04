/*
 * SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
 * SPDX-License-Identifier: Apache-2.0
 */

pub use etcd_client::{Client, DeleteOptions, Error, GetOptions};

pub fn open_server() -> String {
    format!("{}:2379", crate::get_conf("HOST_IP"))
}

async fn get_client() -> Result<Client, Error> {
    Client::connect([open_server()], None).await
}

pub async fn put(key: &str, value: &str) -> Result<(), Error> {
    let mut client = get_client().await?;
    client.put(key, value, None).await?;
    Ok(())
}

pub async fn get(key: &str) -> Result<String, Error> {
    let mut client = get_client().await?;
    let resp = client.get(key, None).await?;

    if let Some(kv) = resp.clone().kvs().first() {
        Ok(kv.value_str()?.to_owned())
    } else {
        Err(etcd_client::Error::InvalidArgs("".to_owned()))
    }
}

pub async fn get_all(key: &str) -> Result<(Vec<String>, Vec<String>), Error> {
    let mut client = get_client().await?;
    let option = Some(GetOptions::new().with_prefix());
    let resp = client.get(key, option).await?;

    let mut k = Vec::<String>::new();
    let mut v = Vec::<String>::new();
    for kv in resp.clone().kvs() {
        k.push(kv.key_str()?.to_string());
        v.push(kv.value_str()?.to_string());
    }

    Ok((k, v))
}

pub async fn delete(key: &str) -> Result<(), Error> {
    let mut client = get_client().await?;
    client.delete(key, None).await?;
    Ok(())
}

pub async fn delete_all(key: &str) -> Result<(), Error> {
    let mut client = get_client().await?;
    let option = Some(DeleteOptions::new().with_prefix());
    client.delete(key, option).await?;
    Ok(())
}
