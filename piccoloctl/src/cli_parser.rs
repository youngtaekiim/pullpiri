fn help() -> String {
    println!("Usage - piccoloctl COMMAND [PARAMETERS]");
    println!("Available command");
    println!("  - list: shows node list");
    println!("    usage: piccoloctl list");
    println!("  - list (unit): shows unit list in node");
    println!("    usage: piccoloctl list NODE_NAME");
    println!("  - start: start unit in node");
    println!("    usage: piccoloctl start NODE_NAME UNIT_NAME");
    println!("  - stop: stop unit in node");
    println!("    usage: piccoloctl stop NODE_NAME UNIT_NAME");
    println!("  - restart: restart unit in node");
    println!("    usage: piccoloctl restart NODE_NAME UNIT_NAME");
    println!("  - reload: reload unit in node");
    println!("    usage: piccoloctl reload NODE_NAME UNIT_NAME");
    println!("  - enable: enable unit in node");
    println!("    usage: piccoloctl enable NODE_NAME UNIT_NAME");
    println!("  - disable: disable unit in node");
    println!("    usage: piccoloctl disable NODE_NAME UNIT_NAME");
    "not support".to_string()
}

pub fn check(input: &Vec<String>) -> Result<String, String> {
    match input.len() {
        2 => Ok(format!("{}", input[1])),
        3 => Ok(format!("{}/{}", input[1], input[2])),
        4 => Ok(format!("{}/{}/{}", input[1], input[2], input[3])),
        _ => Err(help()),
    }
}
