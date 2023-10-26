use image::{Luma, GrayImage};
use indicatif::{ProgressBar, ProgressStyle};
use pyo3::prelude::*;
use rayon::prelude::*;
use std::time::Instant;
use imageproc::contours::{self, find_contours};
use imageproc::drawing::{draw_polygon_mut, draw_line_segment_mut};

fn p_as_tuple(point: &imageproc::point::Point<i32>) -> (f32, f32) {
    (point.x as f32, point.y as f32)
}

fn draw_contour(
    contour: &Vec<imageproc::point::Point<i32>>,
    base_img: &mut image::ImageBuffer<Luma<u8>, Vec<u8>>,
    color: u8
) {
    for i in 0..contour.len()-1 {
        let p = contour[i];
        let p2 = contour[i+1];
        draw_line_segment_mut(base_img, p_as_tuple(&p), p_as_tuple(&p2), image::Luma([color]));
    }
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
