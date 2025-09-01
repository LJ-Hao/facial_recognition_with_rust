mod cli;
mod models;
mod processors;
mod utils;

use clap::Parser;
use cli::app::Cli;

fn main() {
    let cli = Cli::parse();

    println!("Input image path: {}", cli.input);
    if let Some(output) = &cli.output {
        println!("Output image path: {}", output);
    }

    // Here you would call your processing logic
    // For example:
    // let image = processors::image_loader::load_image(&cli.input);
    // let detections = processors::face_detector::detect_faces(&image);
    // ... further processing ...
}
