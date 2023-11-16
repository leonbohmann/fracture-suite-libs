use pyo3::prelude::*;
use std::time::Instant;
use rayon::prelude::*;
use indicatif::{ProgressBar, ProgressStyle};
use opencv::prelude::*;
use opencv::core;
use opencv::imgproc;

fn draw_contour(
    contour: &Vec<core::Point>,
    base_img: &mut Mat,
    color: ::Scalar,
) -> opencv::Result<()> {
    let pts: Vec<core::Point> = contour.clone();
    let pts = [pts].as_ref();
    imgproc::polylines(base_img, pts, true, color, 1, imgproc::LINE_8, 0)?;
    Ok(())
}

// Check, if contours made of points are adjacent to each other.
//
// points:
// List of points, where each point is a list of two integers.
// The first integer is the x-coordinate, the second the y-coordinate.
// The points are assumed to be sorted by x-coordinate.
// ids:
// List of ids, where each id is an integer.
// Each id corresponds to a contour and connects points[i] to contour[ids[i]].
#[pyfunction]
pub fn check_contours(points: Vec<Vec<i32>>, ids: Vec<i32>, centers: Vec<Vec<i32>>) {
    println!("Create base image");
    // create new image buffer of size 4000x4000 px
    let mut base_img: GrayImage = image::GrayImage::new(4000, 4000);

    // gather contours from points, that have the same id in ids
    println!("Gathering contours");
    let mut contours = Vec::new();
    let mut contour = vec![imageproc::point::Point::<i32>::new(0,0);0];
    let mut last_id = ids[0];
    for i in 0..points.len() {
        if ids[i] != last_id {
            contours.push(contour);
            contour = Vec::new();
            last_id = ids[i];
        }
        contour.push(imageproc::point::Point::<i32>::new(points[i][0], points[i][1]));
    }

    // draw all contours into the base image
    println!("Drawing contours");
    for contour in contours.iter() {
        draw_contour(contour, &mut base_img, 255);
    }


    for (i, contour) in contours.iter().enumerate(){
        // this is the current contour image
        let mut ctr_img: GrayImage = base_img.clone();
        draw_contour(contour, &mut ctr_img, 0);

        // detect contours in ctr_image
        let ctrs = find_contours::<u8>(&ctr_img);

        // find the ctrs that contains the center of the current contour
        let center = centers[i];

        for ctr in ctrs.iter(){

        }

    }

}
