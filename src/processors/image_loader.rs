/// Loads an image from a file path.
///
/// # Arguments
///
/// * `path` - A string slice that holds the path to the image file.
///
/// # Returns
///
/// * `Ok(image::DynamicImage)` - The loaded image.
/// * `Err(Box<dyn std::error::Error>)` - An error if the image could not be loaded.
pub fn load_image(path: &str) -> Result<image::DynamicImage, Box<dyn std::error::Error>> {
    let img = image::open(path)?;
    Ok(img)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_image_failure() {
        let result = load_image("non_existent_image.png");
        assert!(result.is_err());
    }
}