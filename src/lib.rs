pub mod cli;
pub mod models;
pub mod processors;
pub mod utils;

/// Public API function to process an image and detect faces.
///
/// # Arguments
///
/// * `image_path` - A string slice that holds the path to the image file.
///
/// # Returns
///
/// * `Result<Vec<crate::models::detection::Detection>, Box<dyn std::error::Error>>` - A result containing a vector of detections or an error.
pub fn process_image(
    image_path: &str,
) -> Result<Vec<crate::models::detection::Detection>, Box<dyn std::error::Error>> {
    let image = crate::processors::image_loader::load_image(image_path)?;
    let detections = crate::processors::face_detector::detect_faces(&image);
    Ok(detections)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_process_image_with_invalid_path() {
        let result = process_image("invalid_path.png");
        assert!(result.is_err());
    }
    
    #[test]
    fn test_process_image_with_valid_path() {
        // Create a temporary directory
        let dir = tempdir().expect("Failed to create temporary directory");
        let file_path = dir.path().join("test_image.png");
        
        // Create a simple test image
        let mut img_buffer = image::RgbImage::new(100, 100);
        // Fill with blue color
        for pixel in img_buffer.pixels_mut() {
            *pixel = image::Rgb([0, 0, 255]);
        }
        
        // Save the image
        img_buffer.save(&file_path).expect("Failed to save test image");
        
        // Process the image
        let result = process_image(file_path.to_str().unwrap());
        assert!(result.is_ok());
        
        // Clean up
        dir.close().expect("Failed to clean up temporary directory");
    }
}
