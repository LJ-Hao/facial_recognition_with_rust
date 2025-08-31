//! Database module for facial recognition system
//! 
//! This module handles the storage and retrieval of authorized face records
//! in a local JSON file. Each face record contains the person's name, 
//! photo path, and creation timestamp.

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Represents a single authorized face record in the database
/// 
/// This struct stores information about an authorized person including:
/// - Unique identifier for the record
/// - Person's name
/// - Path to their reference photo
/// - Timestamp when the record was created
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FaceRecord {
    /// Unique identifier for this face record (UUID v4)
    pub id: String,
    /// Name of the authorized person
    pub name: String,
    /// File path to the reference photo for this person
    pub photo_path: String,
    /// UTC timestamp indicating when this record was created
    pub created_at: DateTime<Utc>,
}

impl FaceRecord {
    /// Create a new face record with the given name and photo path
    /// 
    /// # Arguments
    /// * `name` - The name of the authorized person
    /// * `photo_path` - Path to their reference photo
    /// 
    /// # Returns
    /// A new FaceRecord instance with a generated UUID and current timestamp
    pub fn new(name: String, photo_path: String) -> Self {
        Self {
            // Generate a unique UUID for this record
            id: Uuid::new_v4().to_string(),
            name,
            photo_path,
            // Record the current UTC time
            created_at: Utc::now(),
        }
    }
}

/// Manages a collection of authorized face records
/// 
/// This struct provides functionality to load, save, and manage face records
/// stored in a JSON file. It maintains an in-memory vector of FaceRecord instances
/// for quick access during facial recognition operations.
#[derive(Debug, Clone)]
pub struct FaceDatabase {
    /// Collection of authorized face records
    pub records: Vec<FaceRecord>,
}

impl FaceDatabase {
    /// Create a new FaceDatabase instance
    /// 
    /// This function attempts to load existing face records from the JSON file.
    /// If the file doesn't exist, it creates a new empty database.
    /// 
    /// # Returns
    /// Result containing either a FaceDatabase instance or an error
    /// 
    /// # Errors
    /// Returns an error if there are issues reading or parsing the JSON file
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let db_path = "database/face_records.json";
        
        // Check if the database file exists
        if Path::new(db_path).exists() {
            // Read the JSON file content
            let data = fs::read_to_string(db_path)?;
            // Parse the JSON data into FaceRecord vector
            let records: Vec<FaceRecord> = serde_json::from_str(&data)?;
            Ok(FaceDatabase { records })
        } else {
            // Return an empty database if file doesn't exist
            Ok(FaceDatabase {
                records: Vec::new(),
            })
        }
    }
    
    /// Add a new face record to the database
    /// 
    /// This function adds a new FaceRecord to the in-memory collection and 
    /// immediately saves the updated database to the JSON file.
    /// 
    /// # Arguments
    /// * `record` - The FaceRecord to add to the database
    /// 
    /// # Returns
    /// Result indicating success or failure of the operation
    /// 
    /// # Errors
    /// Returns an error if there are issues saving the database to file
    pub fn add_record(&mut self, record: FaceRecord) -> Result<(), Box<dyn std::error::Error>> {
        self.records.push(record);
        self.save()
    }
    
    /// Save the current face database to the JSON file
    /// 
    /// This function serializes the in-memory face records to JSON format
    /// and writes them to the database file.
    /// 
    /// # Returns
    /// Result indicating success or failure of the operation
    /// 
    /// # Errors
    /// Returns an error if there are issues serializing or writing to the file
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let db_path = "database/face_records.json";
        // Serialize records to pretty-printed JSON
        let data = serde_json::to_string_pretty(&self.records)?;
        // Write to file
        fs::write(db_path, data)?;
        Ok(())
    }
    
    /// Get a reference to the authorized faces collection
    /// 
    /// This function provides read-only access to the vector of authorized face records.
    /// 
    /// # Returns
    /// A reference to the vector of FaceRecord instances
    pub fn get_authorized_faces(&self) -> &Vec<FaceRecord> {
        &self.records
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;
    use chrono::Utc;

    /// Test the creation of a new FaceRecord
    /// 
    /// This test verifies that:
    /// 1. A FaceRecord can be created with the correct name and photo path
    /// 2. The record gets a non-empty UUID
    /// 3. The record gets a timestamp
    #[test]
    fn test_face_record_creation() {
        let record = FaceRecord::new("John Doe".to_string(), "database/john.jpg".to_string());
        
        // Verify the record was created with correct values
        assert!(!record.id.is_empty());
        assert_eq!(record.name, "John Doe");
        assert_eq!(record.photo_path, "database/john.jpg");
        // Verify that the timestamp is recent (within the last minute)
        assert!(record.created_at < Utc::now());
        assert!(record.created_at > Utc::now() - chrono::Duration::minutes(1));
    }

    /// Test FaceDatabase operations
    /// 
    /// This test verifies that:
    /// 1. A new database can be created
    /// 2. Records can be added to the database
    /// 3. The database can be saved to file
    /// 4. Records can be retrieved from the database
    #[test]
    fn test_face_database_operations() {
        // Use a test database path to avoid interfering with real data
        let test_db_path = "database/test_face_records.json";
        
        // Create test directory if it doesn't exist
        fs::create_dir_all("database").unwrap();
        
        // Clean up any existing test file
        if Path::new(test_db_path).exists() {
            let _ = fs::remove_file(test_db_path);
        }
        
        // Test creating new database (should be empty)
        let mut face_db = FaceDatabase::new().unwrap();
        assert_eq!(face_db.records.len(), 0);
        
        // Test adding a record
        let record = FaceRecord::new("Jane Doe".to_string(), "database/jane.jpg".to_string());
        face_db.add_record(record).unwrap();
        
        // Verify record was added
        assert_eq!(face_db.records.len(), 1);
        assert_eq!(face_db.records[0].name, "Jane Doe");
        
        // Test retrieving authorized faces
        let authorized_faces = face_db.get_authorized_faces();
        assert_eq!(authorized_faces.len(), 1);
        assert_eq!(authorized_faces[0].name, "Jane Doe");
        
        // Clean up
        if Path::new(test_db_path).exists() {
            let _ = fs::remove_file(test_db_path);
        }
    }

    /// Test saving and loading FaceDatabase from file
    /// 
    /// This test verifies that:
    /// 1. A database can be saved to a JSON file
    /// 2. A database can be loaded from a JSON file
    /// 3. The loaded data matches the saved data
    #[test]
    fn test_face_database_save_load() {
        // Use a test database path
        let test_db_path = "database/test_save_load_face_records.json";
        
        // Create test directory if it doesn't exist
        fs::create_dir_all("database").unwrap();
        
        // Clean up any existing test file
        if Path::new(test_db_path).exists() {
            let _ = fs::remove_file(test_db_path);
        }
        
        // Create a database with some records
        let mut face_db = FaceDatabase {
            records: vec![
                FaceRecord::new("Alice Smith".to_string(), "database/alice.jpg".to_string()),
                FaceRecord::new("Bob Johnson".to_string(), "database/bob.jpg".to_string()),
            ]
        };
        
        // Save the database
        face_db.save().unwrap();
        
        // Verify the file was created
        assert!(Path::new(test_db_path).exists());
        
        // Load a new database from the file
        let loaded_db = FaceDatabase::new().unwrap();
        
        // Verify the loaded database has the correct number of records
        assert_eq!(loaded_db.records.len(), 2);
        
        // Verify the loaded records have the correct names
        let names: Vec<String> = loaded_db.records.iter().map(|r| r.name.clone()).collect();
        assert!(names.contains(&"Alice Smith".to_string()));
        assert!(names.contains(&"Bob Johnson".to_string()));
        
        // Verify that each record has a unique ID
        assert_ne!(loaded_db.records[0].id, loaded_db.records[1].id);
        
        // Verify that each record has a timestamp
        assert!(loaded_db.records[0].created_at < Utc::now());
        assert!(loaded_db.records[1].created_at < Utc::now());
        
        // Clean up test file
        if Path::new(test_db_path).exists() {
            let _ = fs::remove_file(test_db_path);
        }
    }

    /// Test adding multiple records to FaceDatabase
    /// 
    /// This test verifies that:
    /// 1. Multiple records can be added to the database
    /// 2. Each record gets a unique ID
    /// 3. All records are saved correctly
    #[test]
    fn test_face_database_multiple_records() {
        // Use a test database path
        let test_db_path = "database/test_multiple_face_records.json";
        
        // Create test directory if it doesn't exist
        fs::create_dir_all("database").unwrap();
        
        // Clean up any existing test file
        if Path::new(test_db_path).exists() {
            let _ = fs::remove_file(test_db_path);
        }
        
        // Create a new database
        let mut face_db = FaceDatabase::new().unwrap();
        
        // Add multiple records
        let records = vec![
            FaceRecord::new("Person 1".to_string(), "database/person1.jpg".to_string()),
            FaceRecord::new("Person 2".to_string(), "database/person2.jpg".to_string()),
            FaceRecord::new("Person 3".to_string(), "database/person3.jpg".to_string()),
        ];
        
        // Keep track of IDs to ensure uniqueness
        let mut ids = std::collections::HashSet::new();
        
        // Add each record and verify uniqueness
        for record in records {
            assert!(ids.insert(record.id.clone())); // insert returns false if ID already exists
            face_db.add_record(record).unwrap();
        }
        
        // Verify all records were added
        assert_eq!(face_db.records.len(), 3);
        
        // Verify all names are present
        let names: Vec<String> = face_db.records.iter().map(|r| r.name.clone()).collect();
        assert!(names.contains(&"Person 1".to_string()));
        assert!(names.contains(&"Person 2".to_string()));
        assert!(names.contains(&"Person 3".to_string()));
        
        // Clean up test file
        if Path::new(test_db_path).exists() {
            let _ = fs::remove_file(test_db_path);
        }
    }
}