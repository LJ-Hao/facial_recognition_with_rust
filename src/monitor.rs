use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::time::Duration;
use notify::{Watcher, RecursiveMode, PollWatcher, Event};
use tokio::sync::RwLock;
use tokio::time::sleep;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct DatabaseMonitor {
    face_db: crate::database::FaceDatabase,
    photo_files: HashMap<String, u64>, // filename -> modified time
}

#[derive(Serialize, Clone)]
pub struct RecognitionResponse {
    pub name: Option<String>,
    pub recognized: bool,
}

impl DatabaseMonitor {
    pub fn new(face_db: crate::database::FaceDatabase) -> Result<Self, Box<dyn std::error::Error>> {
        let mut monitor = DatabaseMonitor {
            face_db,
            photo_files: HashMap::new(),
        };
        
        // Initial scan of database photos
        monitor.scan_database()?;
        
        Ok(monitor)
    }
    
    pub fn scan_database(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let database_path = "database";
        if !Path::new(database_path).exists() {
            fs::create_dir_all(database_path)?;
            return Ok(());
        }
        
        let mut current_files = HashMap::new();
        
        for entry in fs::read_dir(database_path)? {
            let entry = entry?;
            let path = entry.path();
            
            // Only process JPG files (both .jpg and .jpeg extensions)
            if let Some(extension) = path.extension() {
                let ext = extension.to_string_lossy().to_lowercase();
                if ext == "jpg" || ext == "jpeg" {
                    let file_name = path.file_name().unwrap().to_string_lossy().to_string();
                    let metadata = fs::metadata(&path)?;
                    let modified = metadata.modified()?.duration_since(std::time::UNIX_EPOCH)?.as_secs();
                    
                    current_files.insert(file_name, modified);
                }
            }
        }
        
        // Check for added files
        for (file_name, modified_time) in &current_files {
            if !self.photo_files.contains_key(file_name) {
                println!("New photo added: {}", file_name);
                // In a real implementation, you might want to update the database here
            }
        }
        
        // Check for removed files
        for file_name in self.photo_files.keys() {
            if !current_files.contains_key(file_name) {
                println!("Photo removed: {}", file_name);
                // In a real implementation, you might want to update the database here
            }
        }
        
        self.photo_files = current_files;
        Ok(())
    }
    
    pub fn get_face_database(&self) -> &crate::database::FaceDatabase {
        &self.face_db
    }
    
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
    use warp::Filter;
    
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