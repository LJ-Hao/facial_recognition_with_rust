#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::thread;
    use std::time::Duration;
    use tokio;

    #[test]
    fn test_face_record_creation() {
        let record = FaceRecord::new("John Doe".to_string(), "database/john.png".to_string());
        
        assert!(!record.id.is_empty());
        assert_eq!(record.name, "John Doe");
        assert_eq!(record.photo_path, "database/john.png");
    }

    #[test]
    fn test_face_database_operations() {
        // Use a test database path
        let test_db_path = "database/test_face_records.json";
        
        // Create test directory if it doesn't exist
        fs::create_dir_all("database").unwrap();
        
        // Clean up any existing test file
        if std::path::Path::new(test_db_path).exists() {
            let _ = fs::remove_file(test_db_path);
        }
        
        // Test creating new database
        let mut face_db = FaceDatabase::new().unwrap();
        assert_eq!(face_db.records.len(), 0);
        
        // Test adding a record
        let record = FaceRecord::new("Jane Doe".to_string(), "database/jane.png".to_string());
        face_db.add_record(record).unwrap();
        
        // Verify record was added
        assert_eq!(face_db.records.len(), 1);
        assert_eq!(face_db.records[0].name, "Jane Doe");
        
        // Clean up
        if std::path::Path::new(test_db_path).exists() {
            let _ = fs::remove_file(test_db_path);
        }
    }
}