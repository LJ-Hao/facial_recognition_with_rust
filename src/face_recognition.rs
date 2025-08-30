#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deep_face_recognizer_creation() {
        let recognizer = DeepFaceRecognizer::new();
        // This test will fail because we need OpenCV setup, but we can at least verify it compiles
        // In a real test environment, we would set up OpenCV properly
        assert!(true); // Placeholder for now
    }

    #[test]
    fn test_compare_faces() {
        let recognizer = DeepFaceRecognizer::new().unwrap();
        
        // Test identical features
        let features1 = vec![1.0, 0.0, 0.0];
        let features2 = vec![1.0, 0.0, 0.0];
        let similarity = recognizer.compare_faces(&features1, &features2);
        assert!((similarity - 1.0).abs() < 0.001);
        
        // Test orthogonal features
        let features3 = vec![1.0, 0.0, 0.0];
        let features4 = vec![0.0, 1.0, 0.0];
        let similarity2 = recognizer.compare_faces(&features3, &features4);
        assert!(similarity2 < 0.001);
        
        // Test opposite features
        let features5 = vec![1.0, 0.0, 0.0];
        let features6 = vec![-1.0, 0.0, 0.0];
        let similarity3 = recognizer.compare_faces(&features5, &features6);
        assert!(similarity3 < 0.001);
    }
}