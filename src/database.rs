use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FaceRecord {
    pub id: String,
    pub name: String,
    pub photo_path: String,
    pub created_at: DateTime<Utc>,
}

impl FaceRecord {
    pub fn new(name: String, photo_path: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            photo_path,
            created_at: Utc::now(),
        }
    }
}

pub struct FaceDatabase {
    pub records: Vec<FaceRecord>,
}

impl FaceDatabase {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let db_path = "database/face_records.json";
        
        if Path::new(db_path).exists() {
            let data = fs::read_to_string(db_path)?;
            let records: Vec<FaceRecord> = serde_json::from_str(&data)?;
            Ok(FaceDatabase { records })
        } else {
            Ok(FaceDatabase {
                records: Vec::new(),
            })
        }
    }
    
    pub fn add_record(&mut self, record: FaceRecord) -> Result<(), Box<dyn std::error::Error>> {
        self.records.push(record);
        self.save()
    }
    
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let db_path = "database/face_records.json";
        let data = serde_json::to_string_pretty(&self.records)?;
        fs::write(db_path, data)?;
        Ok(())
    }
    
    pub fn get_authorized_faces(&self) -> &Vec<FaceRecord> {
        &self.records
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

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
        if Path::new(test_db_path).exists() {
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
    }
}