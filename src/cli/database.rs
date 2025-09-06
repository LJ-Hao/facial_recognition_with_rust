use std::fs;
use std::path::Path;

/// Represents a person in the database
pub struct Person {
    pub name: String,
    pub image_path: String,
}

/// Loads the database of known faces from a directory
///
/// # Arguments
///
/// * `database_path` - Path to the directory containing reference images
///
/// # Returns
///
/// * `Vec<Person>` - A vector of Person objects representing the database
pub fn load_database(database_path: &str) -> Result<Vec<Person>, Box<dyn std::error::Error>> {
    let mut database = Vec::new();

    // Check if the database directory exists
    if !Path::new(database_path).exists() {
        return Err(format!("Database directory '{}' does not exist", database_path).into());
    }

    // Read all files in the database directory
    for entry in fs::read_dir(database_path)? {
        let entry = entry?;
        let path = entry.path();

        // Only process files with .jpg extension
        if path.is_file() && path.extension().is_some_and(|ext| ext == "jpg") {
            if let Some(file_name) = path.file_stem() {
                let name = file_name.to_string_lossy().to_string();
                let image_path = path.to_string_lossy().to_string();

                database.push(Person { name, image_path });
            }
        }
    }

    Ok(database)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_load_database() {
        // Create a temporary directory for testing
        let temp_dir = tempfile::tempdir().expect("Failed to create temp directory");
        let db_path = temp_dir.path().to_str().unwrap();

        // Create some test images
        let file1_path = temp_dir.path().join("person1.jpg");
        let mut file1 = File::create(&file1_path).expect("Failed to create test file");
        file1
            .write_all(b"fake image data")
            .expect("Failed to write to test file");

        let file2_path = temp_dir.path().join("person2.jpg");
        let mut file2 = File::create(&file2_path).expect("Failed to create test file");
        file2
            .write_all(b"fake image data")
            .expect("Failed to write to test file");

        // Load the database
        let database = load_database(db_path).expect("Failed to load database");

        // Check that we have the correct number of entries
        assert_eq!(database.len(), 2);

        // Check that the entries have the correct names
        let names: Vec<&str> = database.iter().map(|p| p.name.as_str()).collect();
        assert!(names.contains(&"person1"));
        assert!(names.contains(&"person2"));
    }

    #[test]
    fn test_load_database_nonexistent() {
        let result = load_database("/nonexistent/path");
        assert!(result.is_err());
    }
}
