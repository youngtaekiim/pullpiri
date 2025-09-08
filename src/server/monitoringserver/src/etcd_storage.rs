/*
 * SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Store and retrieve monitoring data in etcd

use crate::data_structures::{BoardInfo, SocInfo};
use common::monitoringserver::NodeInfo;

/// Store NodeInfo in etcd
pub async fn store_node_info(node_info: &NodeInfo) -> common::Result<()> {
    let key = format!("monitoring/nodes/{}", node_info.node_name);
    let json_data = serde_json::to_string(node_info)
        .map_err(|e| format!("Failed to serialize NodeInfo: {}", e))?;

    common::etcd::put(&key, &json_data).await?;
    println!("[ETCD] Stored NodeInfo for node: {}", node_info.node_name);
    Ok(())
}

/// Store SocInfo in etcd
pub async fn store_soc_info(soc_info: &SocInfo) -> common::Result<()> {
    let key = format!("monitoring/socs/{}", soc_info.soc_id);
    let json_data = serde_json::to_string(soc_info)
        .map_err(|e| format!("Failed to serialize SocInfo: {}", e))?;

    common::etcd::put(&key, &json_data).await?;
    println!("[ETCD] Stored SocInfo for SoC: {}", soc_info.soc_id);
    Ok(())
}

/// Store BoardInfo in etcd
pub async fn store_board_info(board_info: &BoardInfo) -> common::Result<()> {
    let key = format!("monitoring/boards/{}", board_info.board_id);
    let json_data = serde_json::to_string(board_info)
        .map_err(|e| format!("Failed to serialize BoardInfo: {}", e))?;

    common::etcd::put(&key, &json_data).await?;
    println!("[ETCD] Stored BoardInfo for board: {}", board_info.board_id);
    Ok(())
}

/// Retrieve NodeInfo from etcd
pub async fn get_node_info(node_name: &str) -> common::Result<NodeInfo> {
    let key = format!("monitoring/nodes/{}", node_name);
    let json_data = common::etcd::get(&key).await?;

    let node_info: NodeInfo = serde_json::from_str(&json_data)
        .map_err(|e| format!("Failed to deserialize NodeInfo: {}", e))?;

    Ok(node_info)
}

/// Retrieve SocInfo from etcd
pub async fn get_soc_info(soc_id: &str) -> common::Result<SocInfo> {
    let key = format!("monitoring/socs/{}", soc_id);
    let json_data = common::etcd::get(&key).await?;

    let soc_info: SocInfo = serde_json::from_str(&json_data)
        .map_err(|e| format!("Failed to deserialize SocInfo: {}", e))?;

    Ok(soc_info)
}

/// Retrieve BoardInfo from etcd
pub async fn get_board_info(board_id: &str) -> common::Result<BoardInfo> {
    let key = format!("monitoring/boards/{}", board_id);
    let json_data = common::etcd::get(&key).await?;

    let board_info: BoardInfo = serde_json::from_str(&json_data)
        .map_err(|e| format!("Failed to deserialize BoardInfo: {}", e))?;

    Ok(board_info)
}

/// Get all nodes from etcd
pub async fn get_all_nodes() -> common::Result<Vec<NodeInfo>> {
    let kv_pairs = common::etcd::get_all_with_prefix("monitoring/nodes/").await?;

    let mut nodes = Vec::with_capacity(kv_pairs.len());
    for kv in kv_pairs {
        match serde_json::from_str::<NodeInfo>(&kv.value) {
            Ok(node_info) => nodes.push(node_info),
            Err(e) => eprintln!("[ETCD] Failed to deserialize node {}: {}", kv.key, e),
        }
    }

    Ok(nodes)
}

/// Get all SoCs from etcd
pub async fn get_all_socs() -> common::Result<Vec<SocInfo>> {
    let kv_pairs = common::etcd::get_all_with_prefix("monitoring/socs/").await?;

    let mut socs = Vec::new();
    for kv in kv_pairs {
        match serde_json::from_str::<SocInfo>(&kv.value) {
            Ok(soc_info) => socs.push(soc_info),
            Err(e) => eprintln!("[ETCD] Failed to deserialize SoC {}: {}", kv.key, e),
        }
    }

    Ok(socs)
}

/// Get all boards from etcd
pub async fn get_all_boards() -> common::Result<Vec<BoardInfo>> {
    let kv_pairs = common::etcd::get_all_with_prefix("monitoring/boards/").await?;

    let mut boards = Vec::new();
    for kv in kv_pairs {
        match serde_json::from_str::<BoardInfo>(&kv.value) {
            Ok(board_info) => boards.push(board_info),
            Err(e) => eprintln!("[ETCD] Failed to deserialize board {}: {}", kv.key, e),
        }
    }

    Ok(boards)
}

/// Delete NodeInfo from etcd
pub async fn delete_node_info(node_name: &str) -> common::Result<()> {
    let key = format!("monitoring/nodes/{}", node_name);
    common::etcd::delete(&key).await?;
    println!("[ETCD] Deleted NodeInfo for node: {}", node_name);
    Ok(())
}

/// Delete SocInfo from etcd
pub async fn delete_soc_info(soc_id: &str) -> common::Result<()> {
    let key = format!("monitoring/socs/{}", soc_id);
    common::etcd::delete(&key).await?;
    println!("[ETCD] Deleted SocInfo for SoC: {}", soc_id);
    Ok(())
}

/// Delete BoardInfo from etcd
pub async fn delete_board_info(board_id: &str) -> common::Result<()> {
    let key = format!("monitoring/boards/{}", board_id);
    common::etcd::delete(&key).await?;
    println!("[ETCD] Deleted BoardInfo for board: {}", board_id);
    Ok(())
}
