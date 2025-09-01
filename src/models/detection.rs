/// Represents the result of a face detection.
pub struct Detection {
    /// Confidence score of the detection.
    pub confidence: f32,

    /// The bounding box of the detected face.
    pub bounding_box: (u32, u32, u32, u32), // (x, y, width, height)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detection_struct() {
        let detection = Detection {
            confidence: 0.95,
            bounding_box: (10, 10, 100, 100),
        };

        assert_eq!(detection.confidence, 0.95);
        assert_eq!(detection.bounding_box, (10, 10, 100, 100));
    }
}
