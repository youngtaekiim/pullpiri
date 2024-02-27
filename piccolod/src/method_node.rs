use dbus::blocking::Connection;
use dbus::Path;
use std::time::Duration;

pub fn list_node_units(node_name: &str) -> Result<(), Box<dyn std::error::Error>> {
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

    for (name, description) in units {
        println!("{} - {}", name, description);
    }

    Ok(())
}
