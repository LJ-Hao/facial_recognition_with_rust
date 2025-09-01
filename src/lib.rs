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

    #[test]
    fn test_process_image_with_invalid_path() {
        let result = process_image("invalid_path.png");
        assert!(result.is_err());
    }
}
