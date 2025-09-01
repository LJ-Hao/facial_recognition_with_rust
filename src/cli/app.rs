use clap::Parser;

/// A simple facial recognition CLI tool
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// Path to the input image
    #[clap(short, long, value_parser)]
    pub input: String,

    /// Path to the output image (optional)
    #[clap(short, long, value_parser)]
    pub output: Option<String>,
}
