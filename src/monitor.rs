//! Monitor module for facial recognition system
//! 
//! This module provides functionality to monitor the database directory for
//! changes in authorized face photos and serves recognition results via HTTP.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::time::Duration;
use notify::{RecursiveMode, PollWatcher};
use tokio::sync::RwLock;
use tokio::time::sleep;
use std::sync::Arc;
use warp::Filter;

/// Monitors the database directory for changes in authorized face photos
/// 
/// This struct keeps track of the current state of the database directory
/// and detects when new photos are added or existing photos are removed.
/// It maintains a mapping of filenames to their modification times.
#[derive(Debug, Clone)]
pub struct DatabaseMonitor {
    /// Reference to the face database
    face_db: crate::database::FaceDatabase,
    /// Map of photo filenames to their modification times
    photo_files: HashMap<String, u64>, // filename -> modified time
}

/// Represents the response structure for recognition results
/// 
/// This struct is used to serialize recognition results to JSON format
/// for HTTP responses. It includes:
/// - The recognized person's name (None if not recognized)
/// - A boolean indicating whether a face was recognized
#[derive(Serialize, Clone)]
pub struct RecognitionResponse {
    /// Name of the recognized person, or None if not recognized
    pub name: Option<String>,
    /// Boolean indicating whether a face was recognized
    pub recognized: bool,
}

impl DatabaseMonitor {
    /// Create a new DatabaseMonitor instance
    /// 
    /// This function creates a new DatabaseMonitor with the provided FaceDatabase
    /// and performs an initial scan of the database directory.
    /// 
    /// # Arguments
    /// * `face_db` - The FaceDatabase to monitor
    /// 
    /// # Returns
    /// Result containing either a DatabaseMonitor instance or an error
    /// 
    /// # Errors
    /// Returns an error if there are issues scanning the database directory
    pub fn new(face_db: crate::database::FaceDatabase) -> Result<Self, Box<dyn std::error::Error>> {
        // Create a new DatabaseMonitor with empty photo_files map
        let mut monitor = DatabaseMonitor {
            face_db,
            photo_files: HashMap::new(),
        };
        
        // Perform initial scan of database photos
        monitor.scan_database()?;
        
        Ok(monitor)
    }
    
    /// Scan the database directory for changes in photo files
    /// 
    /// This function scans the database directory and compares the current
    /// state with the previously recorded state to detect:
    /// 1. New photos added to the directory
    /// 2. Existing photos removed from the directory
    /// 
    /// # Returns
    /// Result indicating success or failure of the operation
    /// 
    /// # Errors
    /// Returns an error if there are issues reading the directory or file metadata
    pub fn scan_database(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Define the database directory path
        let database_path = "database";
        
        // Create database directory if it doesn't exist
        if !Path::new(database_path).exists() {
            fs::create_dir_all(database_path)?;
            return Ok(());
        }
        
        // Map to store current files and their modification times
        let mut current_files = HashMap::new();
        
        // Iterate through all entries in the database directory
        for entry in fs::read_dir(database_path)? {
            let entry = entry?;
            let path = entry.path();
            
            // Only process JPG files (both .jpg and .jpeg extensions)
            if let Some(extension) = path.extension() {
                let ext = extension.to_string_lossy().to_lowercase();
                if ext == "jpg" || ext == "jpeg" {
                    // Extract filename from path
                    let file_name = path.file_name().unwrap().to_string_lossy().to_string();
                    // Get file metadata to retrieve modification time
                    let metadata = fs::metadata(&path)?;
                    // Convert modification time to seconds since UNIX epoch
                    let modified = metadata.modified()?.duration_since(std::time::UNIX_EPOCH)?.as_secs();
                    
                    // Add file to current files map
                    current_files.insert(file_name, modified);
                }
            }
        }
        
        // Check for added files by comparing current files with previous state
        for (file_name, _modified_time) in &current_files {
            if !self.photo_files.contains_key(file_name) {
                println!("New photo added: {}", file_name);
                // In a real implementation, you might want to update the database here
            }
        }
        
        // Check for removed files by comparing previous state with current files
        for file_name in self.photo_files.keys() {
            if !current_files.contains_key(file_name) {
                println!("Photo removed: {}", file_name);
                // In a real implementation, you might want to update the database here
            }
        }
        
        // Update the photo_files map with current state
        self.photo_files = current_files;
        Ok(())
    }
    
    /// Get a reference to the face database
    /// 
    /// This function provides read-only access to the FaceDatabase instance.
    /// 
    /// # Returns
    /// A reference to the FaceDatabase instance
    pub fn get_face_database(&self) -> &crate::database::FaceDatabase {
        &self.face_db
    }
    
    /// Update the face database with the latest data
    /// 
    /// This function reloads the FaceDatabase from the JSON file to ensure
    /// it contains the most recent authorized face records.
    /// 
    /// # Returns
    /// Result indicating success or failure of the operation
    /// 
    /// # Errors
    /// Returns an error if there are issues loading the database from file
    pub fn update_face_database(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.face_db = crate::database::FaceDatabase::new()?;
        Ok(())
    }
}

// Start the database monitoring task
pub async fn start_database_monitor(
    monitor: Arc<RwLock<DatabaseMonitor>>
) -> Result<(), Box<dyn std::error::Error>> {
    tokio::spawn(async move {
        loop {
            // Scan database every minute
            sleep(Duration::from_secs(60)).await;
            
            let mut monitor = monitor.write().await;
            if let Err(e) = monitor.scan_database() {
                eprintln!("Error scanning database: {}", e);
            }
        }
    });
    
    Ok(())
}

// Start the HTTP server
pub async fn start_http_server(
    recognition_result: Arc<RwLock<RecognitionResponse>>
) -> Result<(), Box<dyn std::error::Error>> {
    // Health check endpoint
    let health_route = warp::path("health")
        .map(|| warp::reply::json(&"OK"));
    
    // Recognition result endpoint
    let result_clone = recognition_result.clone();
    let recognition_route = warp::path("recognition")
        .and(with_recognition_result(result_clone))
        .and_then(handle_recognition_request);
    
    let routes = health_route.or(recognition_route);
    
    println!("Starting HTTP server on port 8001");
    warp::serve(routes)
        .run(([127, 0, 0, 1], 8001))
        .await;
    
    Ok(())
}

// Helper function to pass recognition result to handlers
fn with_recognition_result(
    result: Arc<RwLock<RecognitionResponse>>
) -> impl warp::Filter<Extract = (Arc<RwLock<RecognitionResponse>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || result.clone())
}

// Handler for recognition requests
async fn handle_recognition_request(
    result: Arc<RwLock<RecognitionResponse>>
) -> Result<impl warp::Reply, warp::Rejection> {
    let response = result.read().await.clone();
    Ok(warp::reply::json(&response))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::thread;
    use std::time::Duration;
    use tokio;

    #[test]
    fn test_database_monitor_creation() {
        // Create a temporary face database for testing
        let face_db = crate::database::FaceDatabase::new().unwrap();
        let monitor = DatabaseMonitor::new(face_db).unwrap();
        
        assert_eq!(monitor.photo_files.len(), 0);
    }

    #[test]
    fn test_recognition_response_serialization() {
        let response = RecognitionResponse {
            name: Some("John Doe".to_string()),
            recognized: true,
        };
        
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"name\":\"John Doe\""));
        assert!(json.contains("\"recognized\":true"));
        
        let response_none = RecognitionResponse {
            name: None,
            recognized: false,
        };
        
        let json_none = serde_json::to_string(&response_none).unwrap();
        assert!(json_none.contains("\"name\":null"));
        assert!(json_none.contains("\"recognized\":false"));
    }
}