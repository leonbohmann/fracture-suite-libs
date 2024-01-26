use pyo3::prelude::*;
use rayon::prelude::*;
use std::sync::atomic::{AtomicU64, Ordering};

#[pyfunction]
pub fn calculate_fracture_surface(
    contours: Vec<Vec<Vec<f64>>>,
    image: Vec<Vec<u8>>,
    thickness: f64,
    px_p_mm: f64
) -> f64 {
    // println!("This is the Rust Core: Calculating fracture surface");

	// let iterations_done: AtomicU64 = AtomicU64::new(1_u64);
    // let total_iterations = contours.len() as u64;

    contours.par_iter().map(|contour| {
        // let current_iterations_done = iterations_done.fetch_add(1_u64, Ordering::SeqCst);
        // let frac_surface = calculate_contour_fracsurface(contour, &image, &thickness, &px_p_mm);
        // if current_iterations_done % 100 == 0 {
        //     println!("{}/{} iterations done", current_iterations_done, total_iterations);
        // }
        // frac_surface

        calculate_contour_fracsurface(contour, &image, &thickness, &px_p_mm)


    }).sum()
}

fn dist(a: &(f64, f64), b: &(f64, f64)) -> f64 {
    ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)).sqrt()
}

fn con(a: &(f64, f64), b: &(f64, f64)) -> (f64, f64) {
    let dx = b.0 - a.0;
    let dy = b.1 - a.1;
    (dx, dy)
}

fn perp(a: &(f64, f64), b: &(f64, f64)) -> (f64, f64) {
    let dx = b.0 - a.0;
    let dy = b.1 - a.1;
    (-dy, dx)
}

fn perpv(a: &(f64, f64)) -> (f64, f64) {
    (-a.1, a.0)
}

fn norm(a: &(f64, f64)) -> f64 {
    (a.0.powi(2) + a.1.powi(2)).sqrt()
}

fn ivec(a: &(f64, f64)) -> (i32, i32) {
    (a.0.round() as i32, a.1.round() as i32)
}

fn normed(a: &(f64, f64)) -> (f64, f64) {
    let norm = norm(a);
    (a.0 / norm, a.1 / norm)
}

fn vplus(a: &(f64, f64), b: &(f64, f64)) -> (f64, f64) {
    (a.0 + b.0, a.1 + b.1)
}

fn vplusf(a: &(f64, f64), b: &(f64, f64), f: f64) -> (f64, f64) {
    (a.0 + b.0 * f, a.1 + b.1 * f)
}

fn vtot(a: &Vec<f64>) -> (f64, f64) {
    (a[0], a[1])
}
/// function that takes a list of points, an image and a thickness and returns a float
/// representing the fracture surface
fn calculate_contour_fracsurface(
    points: &[Vec<f64>],
    image: &Vec<Vec<u8>>,
    thickness: &f64,
    px_p_mm: &f64
) -> f64 {

    let mut fracture_surface = 0.0;

    // iterate to get two consecutive points
    for i in 0..points.len() - 1 {
        let p0 = &vtot(&points[i]);
        let p1 = &vtot(points.get(i + 1).unwrap_or(&points[0]));

        // get the vector between the two points
        let v = con(p0, p1);
        let v_norm = norm(&v);

        // get the perpendicular vector and normalize it
        let pv = normed(&perpv(&v));

        let mut lstar1 = 0.0;
        let mut lstar2 = 0.0;
        let mut lstar1done = false;
        let mut lstar2done = false;

        for i in 0..50 {
            if !lstar1done {
                let pl1 = vplusf(p0, &pv, i as f64);
                let pl1i = ivec(&pl1);

                // check if the point is inside the image
                if pl1i.0 < 0 || pl1i.0 >= image.len() as i32
                || pl1i.1 < 0 || pl1i.1 >= image[0].len() as i32 {
                    lstar1done = true;
                    continue;
                }

                // if pixel in image is black, set lstar1 to current length
                if image[pl1i.0 as usize][pl1i.1 as usize] == 0 {
                    lstar1 = norm(&pl1);
                } else {
                    lstar1done = true;
                }
            } else if !lstar2done {
                let pl2 = vplusf(p0, &pv, -i as f64);
                let pl2i = ivec(&pl2);

                // check if the point is inside the image
                if pl2i.0 < 0 || pl2i.0 >= image.len() as i32
                || pl2i.1 < 0 || pl2i.1 >= image[0].len() as i32 {
                    lstar2done = true;
                    continue;
                }

                // if pixel in image is black, set lstar1 to current length
                if image[pl2i.0 as usize][pl2i.1 as usize] == 0 {
                    lstar2 = norm(&pl2);
                } else {
                    lstar2done = true;
                }
            } else {
                break;
            }
        }

        let lstar = (lstar1 + lstar2) / px_p_mm;
        let lm = lstar + thickness;
        let ld = f64::sqrt(thickness.powi(2) + lstar.powi(2));
        fracture_surface += 0.5 * (lm+ld)/2.0 * v_norm / px_p_mm;
    }

    fracture_surface
}