use image::{ImageBuffer, Rgb, ImageFormat};
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

fn main() {
    // Create a new RGB image buffer with a width of 100 and height of 100
    let mut img = ImageBuffer::new(100, 100);

    // Fill the image with a gradient
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let r = (x as f32 / 100.0 * 255.0) as u8;
        let g = (y as f32 / 100.0 * 255.0) as u8;
        let b = 128;
        *pixel = Rgb([r, g, b]);
    }

    // Save the image as a PNG file
    let file_path = "test_image.png";
    let file = File::create(file_path).expect("Failed to create file");
    let writer = BufWriter::new(file);
    img.write_to(writer, ImageFormat::Png).expect("Failed to write image");

    println!("Test image created at {}", file_path);
}