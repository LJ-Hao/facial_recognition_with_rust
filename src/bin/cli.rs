use clap::{Parser, Subcommand};
use facial_recognition_system::{FaceDatabase, FaceRecord, PhotoDatabase, CustomerPhoto};
use std::fs;
use tokio;

#[derive(Parser)]
#[clap(name = "facial-recognition-cli")]
#[clap(about = "CLI tool for facial recognition system", long_about = None)]
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
        
        /// Path to the photo (should be in database folder with .jpg extension)
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
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    
    // Create database directory if it doesn't exist
    fs::create_dir_all("database")?;
    
    match &cli.command {
        Commands::Add { name, photo } => {
            // Verify photo exists
            if !std::path::Path::new(photo).exists() {
                eprintln!("Error: Photo file '{}' not found", photo);
                std::process::exit(1);
            }
            
            // Verify photo has .jpg extension
            if !photo.to_lowercase().ends_with(".jpg") && !photo.to_lowercase().ends_with(".jpeg") {
                eprintln!("Error: Photo file must have .jpg or .jpeg extension");
                std::process::exit(1);
            }
            
            let mut face_db = FaceDatabase::new()?;
            let record = FaceRecord::new(name.clone(), photo.clone());
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
                    println!("ID: {} | Name: {} | Photo: {}", record.id, record.name, record.photo_path);
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
            let photo_db = PhotoDatabase::new().await?;
            
            if let Some(customer_name) = name {
                let photos = photo_db.get_customer_photos(customer_name).await?;
                println!("Found {} photos for customer '{}'", photos.len(), customer_name);
                for photo in photos {
                    println!("  - Created: {}", photo.created_at);
                }
            } else {
                let photos = photo_db.get_all_photos().await?;
                println!("Found {} customer photos in database", photos.len());
                for photo in photos {
                    println!("  - Customer: {} | Created: {}", photo.customer_name, photo.created_at);
                }
            }
        }
    }
    
    Ok(())
}