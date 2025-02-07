/*
 * SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
 * SPDX-License-Identifier: Apache-2.0
 */

use common::Result;
use dbus::blocking::{Connection, Proxy};
use dbus::Path;
use std::time::Duration;

fn list_nodes(proxy: &Proxy<'_, &Connection>) -> Result<String> {
    let (nodes,): (Vec<(String, dbus::Path, String)>,) =
        proxy.method_call(super::DEST_CONTROLLER, "ListNodes", ())?;

    let mut result = String::new();
    for (name, _, status) in nodes {
        result.push_str(&format!("Node: {}, Status: {}\n", name, status));
    }
    Ok(result)
}

fn reload_all_nodes(proxy: &Proxy<'_, &Connection>) -> Result<String> {
    let (nodes,): (Vec<(String, dbus::Path, String)>,) =
        proxy.method_call(super::DEST_CONTROLLER, "ListNodes", ())?;

    let conn = Connection::new_system()?;
    let mut result = String::new();
    for (node_name, _, _) in nodes {
        let (node,): (Path,) =
            proxy.method_call(super::DEST_CONTROLLER, "GetNode", (&node_name,))?;

        let node_proxy = conn.with_proxy(super::DEST, node, Duration::from_millis(5000));
        node_proxy.method_call::<(), _, _, _>(super::DEST_NODE, "Reload", ())?;

        result.push_str(&format!("Node - {} is reloaded.\n", &node_name));
    }
    Ok(result)
}

pub fn handle(cmd: super::Command, bluechi: &Proxy<'_, &Connection>) -> Result<String> {
    match cmd {
        super::Command::ControllerListNode => list_nodes(bluechi),
        super::Command::ControllerReloadAllNodes => reload_all_nodes(bluechi),
        _ => Err("cannot find command".into()),
    }
}
