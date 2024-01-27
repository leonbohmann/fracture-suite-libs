
/// |a-b|
pub fn dist(a: &(f64, f64), b: &(f64, f64)) -> f64 {
    ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)).sqrt()
}

/// a-b
pub fn con(a: &(f64, f64), b: &(f64, f64)) -> (f64, f64) {
    let dx = b.0 - a.0;
    let dy = b.1 - a.1;
    (dx, dy)
}

/// perpendicular vector
pub fn perp(a: &(f64, f64), b: &(f64, f64)) -> (f64, f64) {
    let dx = b.0 - a.0;
    let dy = b.1 - a.1;
    (-dy, dx)
}

/// perpendicular vector
pub fn perpv(a: &(f64, f64)) -> (f64, f64) {
    (-a.1, a.0)
}

/// |a|
pub fn norm(a: &(f64, f64)) -> f64 {
    (a.0.powi(2) + a.1.powi(2)).sqrt()
}

/// a rounded to i32
pub fn ivec(a: &(f64, f64)) -> (i32, i32) {
    (a.0.round() as i32, a.1.round() as i32)
}

/// a/|a|
pub fn normed(a: &(f64, f64)) -> (f64, f64) {
    let norm = norm(a);
    (a.0 / norm, a.1 / norm)
}

/// a+b
pub fn vplus(a: &(f64, f64), b: &(f64, f64)) -> (f64, f64) {
    (a.0 + b.0, a.1 + b.1)
}

/// a+b*f
pub fn vplusf(a: &(f64, f64), b: &(f64, f64), f: f64) -> (f64, f64) {
    (a.0 + b.0 * f, a.1 + b.1 * f)
}

/// vector to tuple
pub fn vtot(a: &[f64]) -> (f64, f64) {
    (a[0], a[1])
}