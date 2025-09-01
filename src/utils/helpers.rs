/// A simple helper function to calculate the area of a rectangle.
///
/// # Arguments
///
/// * `rect` - A tuple representing the rectangle (x, y, width, height).
///
/// # Returns
///
/// * `u32` - The area of the rectangle.
pub fn calculate_area(rect: (u32, u32, u32, u32)) -> u32 {
    rect.2 * rect.3
}

/// A simple helper function to calculate the distance between two points.
///
/// # Arguments
///
/// * `p1` - A tuple representing the first point (x, y).
/// * `p2` - A tuple representing the second point (x, y).
///
/// # Returns
///
/// * `f32` - The Euclidean distance between the two points.
pub fn calculate_distance(p1: (f32, f32), p2: (f32, f32)) -> f32 {
    ((p1.0 - p2.0).powi(2) + (p1.1 - p2.1).powi(2)).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_area() {
        let rect = (0, 0, 10, 20);
        let area = calculate_area(rect);
        assert_eq!(area, 200);
    }

    #[test]
    fn test_calculate_distance() {
        let p1 = (0.0, 0.0);
        let p2 = (3.0, 4.0);
        let distance = calculate_distance(p1, p2);
        assert_eq!(distance, 5.0);
    }
}
