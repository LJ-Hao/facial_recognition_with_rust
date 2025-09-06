use facial_recognition::process_image;

#[test]
fn test_process_test_image() {
    let image_path = "tests/test.jpg";
    let result = process_image(image_path);

    // Assert that the result is Ok (no error occurred)
    assert!(result.is_ok());

    // Get the detections
    let detections = result.unwrap();

    // For now, just check that we get a result
    // In a real implementation, we might check for specific detections
    // Since our detection function is a placeholder, we expect an empty vector
    assert_eq!(detections.len(), 0);
}

#[test]
fn test_process_nonexistent_image() {
    let image_path = "tests/nonexistent.jpg";
    let result = process_image(image_path);

    // Assert that the result is an error (file not found)
    assert!(result.is_err());
}
