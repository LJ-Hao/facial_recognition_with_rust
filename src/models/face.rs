/// Represents a face with its properties.
pub struct Face {
    /// The bounding box of the face in the image.
    pub bounding_box: (u32, u32, u32, u32), // (x, y, width, height)

    /// Facial landmarks (e.g., eyes, nose, mouth).
    pub landmarks: Vec<(f32, f32)>,

    /// A unique encoding of the face for recognition.
    pub encoding: Vec<f32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_face_struct() {
        let face = Face {
            bounding_box: (10, 10, 100, 100),
            landmarks: vec![(50.0, 50.0), (70.0, 50.0)],
            encoding: vec![0.1, 0.2, 0.3],
        };

        assert_eq!(face.bounding_box, (10, 10, 100, 100));
        assert_eq!(face.landmarks.len(), 2);
        assert_eq!(face.encoding.len(), 3);
    }
}
