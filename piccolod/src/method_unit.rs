use dbus::blocking::Connection;
use dbus::Path;
use std::time::Duration;

pub enum Lifecycle {
    Start,
    Stop,
    Restart,
    Reload,
}

pub fn unit_lifecycle(
    life_cycle: Lifecycle,
    node_name: &str,
    unit_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let method: &str = match life_cycle {
        Lifecycle::Start => "StartUnit",
        Lifecycle::Stop => "StopUnit",
        Lifecycle::Restart => "RestartUnit",
        Lifecycle::Reload => "ReloadUnit",
    };
    let conn = Connection::new_system()?;

    let bluechi = conn.with_proxy(
        "org.eclipse.bluechi",
        "/org/eclipse/bluechi",
        Duration::from_millis(5000),
    );

    let (node,): (Path,) =
        bluechi.method_call("org.eclipse.bluechi.Controller", "GetNode", (node_name,))?;

    let node_proxy = conn.with_proxy("org.eclipse.bluechi", node, Duration::from_millis(5000));

    let (job_path,): (Path,) =
        node_proxy.method_call("org.eclipse.bluechi.Node", method, (unit_name, "replace"))?;

    println!("{method} '{unit_name}' on node '{node_name}': {job_path}");

    Ok(())
}

pub fn enable_unit(node_name: &str, unit_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let unit_vector = vec![unit_name.to_string()];
    let conn = Connection::new_system()?;

    let bluechi = conn.with_proxy(
        "org.eclipse.bluechi",
        "/org/eclipse/bluechi",
        Duration::from_millis(5000),
    );

    let (node,): (Path,) =
        bluechi.method_call("org.eclipse.bluechi.Controller", "GetNode", (node_name,))?;

    let node_proxy = conn.with_proxy("org.eclipse.bluechi", node, Duration::from_millis(5000));

    let (carries_install_info, changes): (bool, Vec<(String, String, String)>) = node_proxy
        .method_call(
            "org.eclipse.bluechi.Node",
            "EnableUnitFiles",
            (unit_vector, false, false),
        )?;

    if carries_install_info {
        println!("The unit files included enablement information");
    } else {
        println!("The unit files did not include any enablement information");
    }

    for (op_type, file_name, file_dest) in changes {
        if op_type == "symlink" {
            println!("Created symlink {} -> {}", file_name, file_dest);
        } else if op_type == "unlink" {
            println!("Removed '{}'", file_name);
        }
    }

    Ok(())
}

pub fn disable_unit(node_name: &str, unit_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let unit_vector = vec![unit_name.to_string()];
    let conn = Connection::new_system()?;

    let bluechi = conn.with_proxy(
        "org.eclipse.bluechi",
        "/org/eclipse/bluechi",
        Duration::from_millis(5000),
    );

    let (node,): (Path,) =
        bluechi.method_call("org.eclipse.bluechi.Controller", "GetNode", (node_name,))?;

    let node_proxy = conn.with_proxy("org.eclipse.bluechi", node, Duration::from_millis(5000));

    let (changes,): (Vec<(String, String, String)>,) = node_proxy.method_call(
        "org.eclipse.bluechi.Node",
        "DisableUnitFiles",
        (unit_vector, false),
    )?;

    for (op_type, file_name, file_dest) in changes {
        if op_type == "symlink" {
            println!("Created symlink {} -> {}", file_name, file_dest);
        } else if op_type == "unlink" {
            println!("Removed '{}'", file_name);
        }
    }

    Ok(())
}
