use opencv::{
    core::*,
    highgui::*,
    imgcodecs::*,
    prelude::*,
    videoio::*,
};
use std::fs;
use std::thread;
use std::time::Duration;
use chrono::Utc;
use tokio;

// Import our modules
mod database;
mod face_recognition;
mod photo_db;

use database::FaceDatabase;
use face_recognition::DeepFaceRecognizer;
use photo_db::{PhotoDatabase, CustomerPhoto};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create necessary directories if they don't exist
    fs::create_dir_all("database")?;
    fs::create_dir_all("photos")?;
    
    // Initialize face database
    let face_db = FaceDatabase::new()?;
    
    // Initialize photo database (MongoDB)
    let photo_db = PhotoDatabase::new().await?;
    
    // Initialize deep face recognizer
    let face_recognizer = DeepFaceRecognizer::new()?;
    
    // Initialize camera
    let mut cam = VideoCapture::new(0, VideoCaptureAPIs::CAP_ANY)?; // Open default camera
    if !VideoCapture::is_opened(&cam)? {
        panic!("Cannot open camera");
    }
    
    println!("Facial Recognition System Started with MongoDB Photo Storage");
    
    loop {
        // Capture frame
        let mut frame = Mat::default();
        cam.read(&mut frame)?;
        
        if frame.empty() {
            thread::sleep(Duration::from_secs(1));
            continue;
        }
        
        // Detect faces using deep learning approach
        let faces = face_recognizer.detect_faces(&frame)?;
        
        if !faces.is_empty() {
            // For customer photos (not the 10-second interval photos), save to MongoDB
            // Only save customer photos when a recognized user is detected
            if let Ok(is_recognized) = recognize_face(&frame, &faces, &face_db, &face_recognizer)? {
                if is_recognized {
                    // Convert frame to bytes for MongoDB storage
                    let photo_bytes = mat_to_png_bytes(&frame)?;
                    
                    // Save customer photo to MongoDB (not to file system)
                    // We'll use the first authorized face name for this example
                    if let Some(first_face) = face_db.get_authorized_faces().first() {
                        let customer_photo = CustomerPhoto::new(
                            first_face.name.clone(),
                            photo_bytes
                        );
                        
                        if let Err(e) = photo_db.save_customer_photo(customer_photo).await {
                            eprintln!("Failed to save customer photo to MongoDB: {}", e);
                        } else {
                            println!("Customer photo saved to MongoDB for {}", first_face.name);
                        }
                    }
                    
                    println!("Recognized user - Unlocking screen");
                    unlock_screen()?;
                } else {
                    println!("Unknown face detected - Locking screen");
                    lock_screen()?;
                }
            }
        }
        
        // Wait 10 seconds before next capture (these photos are NOT saved to MongoDB)
        thread::sleep(Duration::from_secs(10));
    }
}

fn mat_to_png_bytes(mat: &Mat) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // Convert Mat to PNG bytes
    let mut buffer = Vector::<u8>::new();
    imencode(".png", mat, &mut buffer, &Vector::new())?;
    Ok(buffer.to_vec())
}

fn save_frame(frame: &Mat, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    imwrite(filename, frame, &Vector::default())?;
    Ok(())
}

fn recognize_face(
    frame: &Mat, 
    faces: &[Rect], 
    face_db: &FaceDatabase, 
    face_recognizer: &DeepFaceRecognizer
) -> Result<bool, Box<dyn std::error::Error>> {
    let authorized_faces = face_db.get_authorized_faces();
    
    if authorized_faces.is_empty() {
        println!("No authorized faces in database");
        return Ok(false);
    }
    
    // For each detected face, compare with authorized faces
    for face_rect in faces {
        // Extract face region
        let face_mat = Mat::roi(frame, *face_rect)?;
        
        // Extract features for detected face
        let detected_features = face_recognizer.extract_features(&face_mat)?;
        
        // Compare with each authorized face
        for record in authorized_faces {
            // Load authorized face image
            if std::path::Path::new(&record.photo_path).exists() {
                let authorized_face = imread(&record.photo_path, IMREAD_COLOR)?;
                
                // Extract features for authorized face
                let authorized_features = face_recognizer.extract_features(&authorized_face)?;
                
                // Compare features
                let similarity = face_recognizer.compare_faces(&detected_features, &authorized_features);
                
                // If similarity is above threshold, we have a match
                if similarity > 0.7 {  // 70% similarity threshold
                    println!("Match found with {} (similarity: {:.2}%)", record.name, similarity * 100.0);
                    return Ok(true);
                }
            }
        }
    }
    
    Ok(false)
}

fn lock_screen() -> Result<(), Box<dyn std::error::Error>> {
    // Platform-specific screen lock command
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-screensaver")
            .arg("lock")
            .output()?;
    }
    
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("pmset")
            .arg("displaysleepnow")
            .output()?;
    }
    
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("rundll32.exe")
            .arg("user32.dll,LockWorkStation")
            .output()?;
    }
    
    Ok(())
}

fn unlock_screen() -> Result<(), Box<dyn std::error::Error>> {
    // Platform-specific screen unlock would be implemented here
    // Note: Unlocking screen programmatically is generally restricted for security reasons
    println!("Screen unlocked - user can now access the system");
    Ok(())
}