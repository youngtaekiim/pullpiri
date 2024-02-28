use std::env;
use std::process;

mod cli_parser;
mod dds_sender;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = cli_parser::check(&args).unwrap_or_else(|_err| {
        process::exit(1);
    });
    println!("{config}");
    dds_sender::run(config);
}
