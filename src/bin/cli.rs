use clap::{Parser, Subcommand};
use facial_recognition_system::{FaceDatabase, FaceRecord, PhotoDatabase};
use std::fs;
use std::path::Path;

#[derive(Parser)]
#[clap(name = "facial-recognition-cli")]
#[clap(about = "CLI tool for facial recognition system", long_about = None)]
#[clap(version = "1.0")]
#[clap(author = "Your Name")]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new authorized face
    Add {
        /// Name of the person
        #[clap(short, long)]
        name: String,

        /// Path to the photo
        #[clap(short, long)]
        photo: String,
    },

    /// List all authorized faces
    List,

    /// Remove an authorized face
    Remove {
        /// ID of the face record to remove
        #[clap(short, long)]
        id: String,
    },

    /// Train the facial recognition model
    Train,

    /// List all customer photos from MongoDB
    ListPhotos {
        /// Optional customer name to filter photos
        #[clap(short, long)]
        name: Option<String>,
    },

    /// Clear all authorized faces
    Clear,

    /// Show system status
    Status,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Create database directory if it doesn't exist
    fs::create_dir_all("database")?;

    match &cli.command {
        Commands::Add { name, photo } => {
            // Verify photo exists
            if !Path::new(photo).exists() {
                eprintln!("Error: Photo file '{}' not found", photo);
                std::process::exit(1);
            }

            // Verify photo has .jpg extension
            let photo_path = Path::new(photo);
            if let Some(extension) = photo_path.extension() {
                let ext = extension.to_string_lossy().to_lowercase();
                if ext != "jpg" && ext != "jpeg" {
                    eprintln!("Error: Photo file must have .jpg or .jpeg extension");
                    std::process::exit(1);
                }
            } else {
                eprintln!("Error: Photo file must have .jpg or .jpeg extension");
                std::process::exit(1);
            }

            // Copy photo to database folder if it's not already there
            let filename = photo_path.file_name().unwrap().to_string_lossy();
            let destination = format!("database/{}", filename);

            if !Path::new(&destination).exists() {
                fs::copy(photo, &destination)?;
                println!("Photo copied to database folder: {}", destination);
            } else {
                println!("Photo already exists in database folder: {}", destination);
            }

            let mut face_db = FaceDatabase::new()?;
            let record = FaceRecord::new(name.clone(), destination);
            face_db.add_record(record)?;
            println!("Successfully added {} to authorized faces", name);
        }
        Commands::List => {
            let face_db = FaceDatabase::new()?;
            if face_db.records.is_empty() {
                println!("No authorized faces in database");
            } else {
                println!("Authorized faces:");
                for record in &face_db.records {
                    println!(
                        "ID: {} | Name: {} | Photo: {}",
                        record.id, record.name, record.photo_path
                    );
                }
            }
        }
        Commands::Remove { id } => {
            let mut face_db = FaceDatabase::new()?;
            let initial_count = face_db.records.len();
            face_db.records.retain(|record| record.id != *id);

            if face_db.records.len() < initial_count {
                face_db.save()?;
                println!("Successfully removed face with ID: {}", id);
            } else {
                println!("No face found with ID: {}", id);
            }
        }
        Commands::Train => {
            println!("Training facial recognition model...");
            // In a more advanced implementation, this would retrain the model
            // For now, we're using pre-trained models
            println!("Model uses pre-trained deep learning models.");
            println!("To add new authorized faces, use the 'add' command.");
        }
        Commands::ListPhotos { name } => {
            let photo_db = PhotoDatabase::new()?;

            if let Some(customer_name) = name {
                let photos = photo_db.get_customer_photos(customer_name)?;
                println!(
                    "Found {} photos for customer '{}'",
                    photos.len(),
                    customer_name
                );
                for photo in photos {
                    println!("  - Created: {}", photo.created_at);
                }
            } else {
                let photos = photo_db.get_all_photos()?;
                println!("Found {} customer photos in database", photos.len());
                for photo in photos {
                    println!(
                        "  - Customer: {} | Created: {}",
                        photo.customer_name, photo.created_at
                    );
                }
            }
        }
        Commands::Clear => {
            let mut face_db = FaceDatabase::new()?;
            let count = face_db.records.len();
            face_db.records.clear();
            face_db.save()?;
            println!(
                "Successfully cleared {} authorized faces from database",
                count
            );
        }
        Commands::Status => {
            let face_db = FaceDatabase::new()?;
            println!("Facial Recognition System Status:");
            println!("  - Authorized faces: {}", face_db.records.len());
            println!("  - Database path: database/face_records.json");

            // Check if database directory exists
            if Path::new("database").exists() {
                println!("  - Database directory: OK");
            } else {
                println!("  - Database directory: MISSING");
            }

            // Check if photos directory exists
            if Path::new("photos").exists() {
                println!("  - Photos directory: OK");
            } else {
                println!("  - Photos directory: MISSING");
            }

            // Check if cascade file exists
            let cascade_path = "haarcascade_frontalface_alt.xml";
            if Path::new(cascade_path).exists() {
                println!("  - Face cascade file: OK");
            } else {
                println!("  - Face cascade file: MISSING (will be downloaded on first run)");
            }
        }
    }

    Ok(())
}
