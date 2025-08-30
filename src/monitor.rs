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