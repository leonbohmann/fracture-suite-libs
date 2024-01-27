
pub fn dist(a: &(f64, f64), b: &(f64, f64)) -> f64 {
    ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)).sqrt()
}

pub fn con(a: &(f64, f64), b: &(f64, f64)) -> (f64, f64) {
    let dx = b.0 - a.0;
    let dy = b.1 - a.1;
    (dx, dy)
}

pub fn perp(a: &(f64, f64), b: &(f64, f64)) -> (f64, f64) {
    let dx = b.0 - a.0;
    let dy = b.1 - a.1;
    (-dy, dx)
}

pub fn perpv(a: &(f64, f64)) -> (f64, f64) {
    (-a.1, a.0)
}

pub fn norm(a: &(f64, f64)) -> f64 {
    (a.0.powi(2) + a.1.powi(2)).sqrt()
}

pub fn ivec(a: &(f64, f64)) -> (i32, i32) {
    (a.0.round() as i32, a.1.round() as i32)
}

pub fn normed(a: &(f64, f64)) -> (f64, f64) {
    let norm = norm(a);
    (a.0 / norm, a.1 / norm)
}

pub fn vplus(a: &(f64, f64), b: &(f64, f64)) -> (f64, f64) {
    (a.0 + b.0, a.1 + b.1)
}

pub fn vplusf(a: &(f64, f64), b: &(f64, f64), f: f64) -> (f64, f64) {
    (a.0 + b.0 * f, a.1 + b.1 * f)
}

pub fn vtot(a: &Vec<f64>) -> (f64, f64) {
    (a[0], a[1])
}