use std::env;
use std::process;

pub mod cli_parser;
pub mod dds_sender;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = cli_parser::check(&args).unwrap_or_else(|err| {
        println!("ERROR: {err}");
        process::exit(1);
    });
    println!("{config}");
    dds_sender::run(config);
}
