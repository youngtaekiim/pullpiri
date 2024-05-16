/*
 * SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
 * SPDX-License-Identifier: Apache-2.0
 */

pub const ETCD_ENDPOINT: &str = const_format::concatcp!(crate::HOST_IP, ":2379");
/*
pub const LISTEN_PEER_URLS: &str = const_format::concatcp!("http://", crate::HOST_IP, ":2380");
pub const LISTEN_CLIENT_URLS: &str = const_format::concatcp!("http://", crate::HOST_IP, ":2379");
pub const ADVERTISE_CLIENT_URLS: &str = const_format::concatcp!("http://", crate::HOST_IP, ":2379");
pub const ETCD_DATA_DIR: &str = "/etcd-data";
*/

pub use etcd_client::{Client, Error};

async fn get_client() -> Result<Client, Error> {
    Client::connect([ETCD_ENDPOINT], None).await
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

pub async fn delete(key: &str) -> Result<(), Error> {
    let mut client = get_client().await?;
    client.delete(key, None).await?;
    Ok(())
}
