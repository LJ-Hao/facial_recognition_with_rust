#[cfg(test)]
mod integration_tests {
    use facial_recognition_system::*;
    use std::fs;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_system_components_integration() {
        // Test that all components can be created
        let face_db = database::FaceDatabase::new().unwrap();
        let monitor = monitor::DatabaseMonitor::new(face_db).unwrap();
        let recognizer = face_recognition::DeepFaceRecognizer::new().unwrap();
        
        assert!(true); // If we get here, all components were created successfully
    }

    #[test]
    fn test_recognition_response_structure() {
        let response = monitor::RecognitionResponse {
            name: Some("Test User".to_string()),
            recognized: true,
        };
        
        assert_eq!(response.name, Some("Test User".to_string()));
        assert_eq!(response.recognized, true);
    }
}