use clap::Parser;

/// This struct represents the arguments
#[derive(Parser, Debug)]
pub struct Arguments {
    /// This is the string argument we are expecting
    pub path: String,
}
