//! Distance related math functions and algorithms

/// Calculate the distance between 2 points. Formula for euclidean distance
/// being `d = √[ (x2 – x1)^2 + (y2 – y1)^2]`
pub fn euclidean_distance(point_a: (f32, f32), point_b: (f32, f32)) -> f32 {
    let (x1, x2) = (point_a.0, point_b.0);
    let (y1, y2) = (point_a.1, point_b.1);

    (((x2 - x1).powi(2)) + ((y2 - y1).powi(2))).sqrt()
}
