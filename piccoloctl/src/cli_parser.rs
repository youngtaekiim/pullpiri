fn help() -> String {
    return "help message".to_string();
}
//TODO - handle each command and parameter
pub fn check(input: &Vec<String>) -> Result<String, String> {
    match input.len() {
        2 => Ok(format!("{}", input[1])),
        3 => Ok(format!("{}/{}", input[1], input[2])),
        4 => Ok(format!("{}/{}/{}", input[1], input[2], input[3])),
        _ => Err(help()),
    }
}
