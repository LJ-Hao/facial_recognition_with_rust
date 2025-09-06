use crate::models::detection::Detection;
use image::{DynamicImage, Pixel};
use std::cmp;

/// Detects faces in an image using a simple skin tone detection algorithm.
///
/// # Arguments
///
/// * `image` - A reference to a `image::DynamicImage`.
///
/// # Returns
///
/// * `Vec<Detection>` - A vector of detected faces.
pub fn detect_faces(image: &DynamicImage) -> Vec<Detection> {
    // Convert the image to grayscale for simpler processing
    let gray_image = image.to_luma8();

    // Get image dimensions
    let (width, height) = gray_image.dimensions();

    // For a simple implementation, we'll look for areas that might be faces
    // based on skin tone detection and size heuristics

    // In a real implementation, we would use a proper face detection algorithm
    // like Haar cascades or a neural network, but for this example we'll implement
    // a basic skin color-based detector

    let mut detections = Vec::new();

    // Simple skin tone detection in RGB space
    // This is a very basic approach - real face detection would be much more sophisticated
    let rgb_image = image.to_rgb8();

    // Define search parameters
    let min_face_size = cmp::max(width, height) / 20; // Minimum face size as 1/20th of image dimension
    let max_face_size = cmp::min(width, height) / 2; // Maximum face size as half of smallest dimension

    // Search for potential face regions
    for y in (0..height).step_by(min_face_size as usize) {
        for x in (0..width).step_by(min_face_size as usize) {
            // Check a region of potential face size
            let region_width = cmp::min(max_face_size, width - x);
            let region_height = cmp::min(max_face_size, height - y);

            if region_width >= min_face_size && region_height >= min_face_size {
                // Analyze skin pixels in this region
                let skin_pixel_count =
                    count_skin_pixels(&rgb_image, x, y, region_width, region_height);
                let total_pixels = region_width * region_height;

                // If a significant portion of pixels are skin-colored, consider it a potential face
                if total_pixels > 0 && (skin_pixel_count as f32 / total_pixels as f32) > 0.3 {
                    // Calculate confidence based on skin pixel ratio
                    let confidence = skin_pixel_count as f32 / total_pixels as f32;

                    detections.push(Detection {
                        confidence,
                        bounding_box: (x, y, region_width, region_height),
                    });
                }
            }
        }
    }

    detections
}

/// Counts skin-colored pixels in a region of an image
fn count_skin_pixels(image: &image::RgbImage, x: u32, y: u32, width: u32, height: u32) -> u32 {
    let mut count = 0;

    // Simple RGB range for skin tones (very basic approximation)
    // In a real implementation, this would be much more sophisticated
    for py in y..(y + height) {
        for px in x..(x + width) {
            if px < image.width() && py < image.height() {
                let pixel = image.get_pixel(px, py);
                let rgb = pixel.channels();
                let r = rgb[0] as f32;
                let g = rgb[1] as f32;
                let b = rgb[2] as f32;

                // Basic skin color check (very simplified)
                // Real face detection would use more advanced techniques
                if r > 95.0
                    && g > 40.0
                    && b > 20.0
                    && r > g
                    && r > b
                    && (r - g) > 15.0
                    && (r - b) > 15.0
                {
                    count += 1;
                }
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{DynamicImage, ImageBuffer, Rgb};

    #[test]
    fn test_detect_faces_empty() {
        // Create a simple test image with no skin-like colors
        let mut img_buffer: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(100, 100);

        // Fill with blue (not skin color)
        for pixel in img_buffer.pixels_mut() {
            *pixel = Rgb([0, 0, 255]);
        }

        let img = DynamicImage::ImageRgb8(img_buffer);
        let detections = detect_faces(&img);

        // With our simple algorithm, we might still get some false positives
        // but with a blue image, we should get very few or none
        assert!(detections.len() <= 1);
    }

    #[test]
    fn test_detect_faces_with_skin_tones() {
        // Create a test image with skin-like colors
        let mut img_buffer: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(200, 200);

        // Fill with skin-like color in a region
        for (x, y, pixel) in img_buffer.enumerate_pixels_mut() {
            if x >= 50 && x < 150 && y >= 50 && y < 150 {
                // Skin-like color in a region
                *pixel = Rgb([180, 140, 120]);
            } else {
                // Non-skin color elsewhere
                *pixel = Rgb([0, 0, 255]);
            }
        }

        let img = DynamicImage::ImageRgb8(img_buffer);
        let detections = detect_faces(&img);

        // Should detect at least one face-like region
        assert!(!detections.is_empty());
    }

    #[test]
    fn test_count_skin_pixels() {
        // Create a test image
        let mut img_buffer: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(10, 10);

        // Fill with skin-like color (some pixels)
        for (i, pixel) in img_buffer.pixels_mut().enumerate() {
            if i % 2 == 0 {
                // Skin-like color
                *pixel = Rgb([200, 150, 130]);
            } else {
                // Non-skin color
                *pixel = Rgb([0, 0, 255]);
            }
        }

        let count = count_skin_pixels(&img_buffer, 0, 0, 10, 10);
        // Should have 50 skin pixels (half of 100)
        assert_eq!(count, 50);
    }

    #[test]
    fn test_count_skin_pixels_no_skin() {
        // Create a test image with no skin pixels
        let mut img_buffer: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(10, 10);

        // Fill with non-skin color
        for pixel in img_buffer.pixels_mut() {
            *pixel = Rgb([0, 0, 255]); // Blue
        }

        let count = count_skin_pixels(&img_buffer, 0, 0, 10, 10);
        // Should have 0 skin pixels
        assert_eq!(count, 0);
    }
}
