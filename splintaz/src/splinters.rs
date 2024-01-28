use pyo3::prelude::*;
use rayon::prelude::*;

use crate::vecs::*;

const MIN_CRACK_WIDTH : i32 = 2;

/// Function to calculate the fracture surface of a list of contours.
///
/// ### Arguments
/// - contours: A list of contours, each contour being a list of points, each point being a list of two floats
/// - image: A two-dimensional array of integers representing the image. 0 is black, 255 is white.
/// - thickness: A float representing the thickness of the splinter
/// - px_p_mm: A float representing the number of pixels per millimeter
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
                //  only if i>2 because the simple crack is 1-2px wide
                if image[pl1i.0 as usize][pl1i.1 as usize] == 0 && i >= MIN_CRACK_WIDTH {
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
                if image[pl2i.0 as usize][pl2i.1 as usize] == 0 && i >= MIN_CRACK_WIDTH {
                    lstar2 = norm(&pl2);
                } else {
                    lstar2done = true;
                }
            } else {
                break;
            }
        }

        let lstar = (lstar1 + lstar2) / px_p_mm;
        let lmax = 2.0*lstar + thickness;
        let lmin = f64::sqrt(thickness.powi(2) + lstar.powi(2));
        // half of the surface to each side of the crack
        fracture_surface += 0.5 * (lmax+lmin)/2.0 * v_norm / px_p_mm;
    }

    fracture_surface
}