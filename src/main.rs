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
use std::sync::Arc;
use tokio::sync::RwLock;

// Import our modules
mod database;
mod face_recognition;
mod photo_db;
mod monitor;

use database::FaceDatabase;
use face_recognition::DeepFaceRecognizer;
use photo_db::{PhotoDatabase, CustomerPhoto};
use monitor::{DatabaseMonitor, RecognitionResponse};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create necessary directories if they don't exist
    fs::create_dir_all("database")?;
    fs::create_dir_all("photos")?;
    
    // Initialize face database
    let face_db = FaceDatabase::new()?;
    
    // Initialize database monitor
    let db_monitor = DatabaseMonitor::new(face_db)?;
    let db_monitor = Arc::new(RwLock::new(db_monitor));
    
    // Initialize photo database (MongoDB)
    let photo_db = PhotoDatabase::new().await?;
    
    // Initialize deep face recognizer
    let face_recognizer = DeepFaceRecognizer::new()?;
    
    // Start database monitoring task
    let monitor_clone = db_monitor.clone();
    tokio::spawn(async move {
        if let Err(e) = monitor::start_database_monitor(monitor_clone).await {
            eprintln!("Database monitoring error: {}", e);
        }
    });
    
    // Keep track of the last recognition result for HTTP responses
    let last_result = Arc::new(RwLock::new(RecognitionResponse {
        name: None,
        recognized: false,
    }));
    
    // Start HTTP server
    let result_clone = last_result.clone();
    tokio::spawn(async move {
        if let Err(e) = monitor::start_http_server(result_clone).await {
            eprintln!("HTTP server error: {}", e);
        }
    });
    
    // Initialize camera
    let mut cam = VideoCapture::new(0, VideoCaptureAPIs::CAP_ANY)?; // Open default camera
    if !VideoCapture::is_opened(&cam)? {
        panic!("Cannot open camera");
    }
    
    println!("Facial Recognition System Started with MongoDB Photo Storage");
    println!("HTTP server running on http://localhost:8001");
    
    loop {
        // Capture frame
        let mut frame = Mat::default();
        cam.read(&mut frame)?;
        
        if frame.empty() {
            thread::sleep(Duration::from_secs(1));
            continue;
        }
        
        // Get current face database from monitor
        let face_db = {
            let monitor = db_monitor.read().await;
            monitor.get_face_database().clone()
        };
        
        // Detect faces using deep learning approach
        let faces = face_recognizer.detect_faces(&frame)?;
        
        if !faces.is_empty() {
            // Save photo with timestamp (these are interval photos, not stored in DB)
            let photo_name = format!("photos/{}.jpg", Utc::now().timestamp());
            save_frame(&frame, &photo_name)?;
            
            // Try to recognize face
            if recognize_face(&frame, &faces, &face_db, &face_recognizer)? {
                println!("Recognized user - Unlocking screen");
                unlock_screen()?;
                
                // Update last recognition result
                {
                    let mut result = last_result.write().await;
                    result.name = Some("Authorized User".to_string()); // In a real implementation, this would be the actual user name
                    result.recognized = true;
                }
                
                // Save customer photo to MongoDB (not interval photos)
                let photo_bytes = mat_to_jpg_bytes(&frame)?;
                let customer_photo = CustomerPhoto::new("Authorized User".to_string(), photo_bytes);
                if let Err(e) = photo_db.save_customer_photo(customer_photo).await {
                    eprintln!("Failed to save customer photo to MongoDB: {}", e);
                } else {
                    println!("Customer photo saved to MongoDB");
                }
            } else {
                println!("Unknown face detected - Locking screen");
                lock_screen()?;
                
                // Update last recognition result
                {
                    let mut result = last_result.write().await;
                    result.name = None;
                    result.recognized = false;
                }
            }
        }
        
        // Wait 10 seconds before next capture
        thread::sleep(Duration::from_secs(10));
    }
}

fn mat_to_jpg_bytes(mat: &Mat) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut buffer = Vector::<u8>::new();
    imencode(".jpg", mat, &mut buffer, &Vector::new())?;
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