mod cli;
mod models;
mod processors;
mod utils;

use clap::Parser;
use cli::app::Cli;
use cli::database;

fn main() {
    let cli = Cli::parse();

    println!("Input image path: {}", cli.input);
    if let Some(output) = &cli.output {
        println!("Output image path: {}", output);
    }
    println!("Database path: {}", cli.database);

    // Load the database of known faces
    match database::load_database(&cli.database) {
        Ok(database) => {
            println!("Loaded {} persons from database", database.len());

            // Print the names of persons in the database
            for person in &database {
                println!("  - {}", person.name);
            }

            // Here you would call your processing logic
            // For example:
            // let image = processors::image_loader::load_image(&cli.input);
            // let detections = processors::face_detector::detect_faces(&image);
            // ... further processing ...
        }
        Err(e) => {
            eprintln!("Error loading database: {}", e);
            std::process::exit(1);
        }
    }
}
