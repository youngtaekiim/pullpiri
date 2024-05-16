/*
 * SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
 * SPDX-License-Identifier: Apache-2.0
 */

use dbus::blocking::Connection;
use dbus::Path;
use std::error::Error;
use std::time::Duration;

fn list_node_units(node_name: &str) -> Result<String, Box<dyn Error>> {
    let conn = Connection::new_system()?;

    let bluechi = conn.with_proxy(
        "org.eclipse.bluechi",
        "/org/eclipse/bluechi",
        Duration::from_millis(5000),
    );

    let (node,): (Path,) =
        bluechi.method_call("org.eclipse.bluechi.Controller", "GetNode", (node_name,))?;

    let node_proxy = conn.with_proxy("org.eclipse.bluechi", node, Duration::from_millis(5000));

    // we are only interested in the first two response values - unit name and description
    let (units,): (Vec<(String, String)>,) =
        node_proxy.method_call("org.eclipse.bluechi.Node", "ListUnits", ())?;

    let mut result = String::new();
    for (name, description) in units {
        result.push_str(&format!("{} - {}\n", name, description));
    }

    Ok(result)
}

#[allow(dead_code)]
fn node_daemon_reload(node_name: &str) -> Result<String, Box<dyn Error>> {
    let conn = Connection::new_system()?;

    let bluechi = conn.with_proxy(
        "org.eclipse.bluechi",
        "/org/eclipse/bluechi",
        Duration::from_millis(5000),
    );

    let (node,): (Path,) =
        bluechi.method_call("org.eclipse.bluechi.Controller", "GetNode", (node_name,))?;

    let node_proxy = conn.with_proxy("org.eclipse.bluechi", node, Duration::from_millis(5000));
    node_proxy.method_call("org.eclipse.bluechi.Node", "Reload", ())?;

    Ok(format!("reload node '{}'\n", node_name))
}

pub fn handle_cmd(c: Vec<&str>) -> Result<String, Box<dyn Error>> {
    match c[0] {
        "LIST_UNIT" => list_node_units(c[1]),
        //"daemon-reload" => node_daemon_reload(c[1]),
        _ => Err("cannot find command".into()),
    }
}
