use pyo3::prelude::*;
use rand::Rng;

fn generate_point(width: f64, height: f64) -> (f64, f64) {
    let mut rng = rand::thread_rng();
    (rng.gen::<f64>() * width, rng.gen::<f64>() * height)
}

fn distance(point1: &(f64, f64), point2: &(f64, f64)) -> f64 {
    ((point1.0 - point2.0).powi(2) + (point1.1 - point2.1).powi(2)).sqrt()
}

#[pyfunction]
pub fn csstraussproc2(width: f64, height: f64, delta: f64, n: usize, c: f64, i_max: i32) -> Vec<(f64, f64)> {
    if delta <= 0.0 {
        panic!("Delta must be positive.");
    }

    if !(0.0..=1.0).contains(&c) {
        panic!("C must be in the interval [0,1].");
    }

    let mut rng = rand::thread_rng();
    let mut points = Vec::with_capacity(n);
    points.push(generate_point(width, height));

    let mut iterations = 0;

    while points.len() < n {
        let candidate = generate_point(width, height);
        let mut too_close = false;
        let mut inhibition_count = 0;


        //TODO: Delta has to be dependent on the distance and the angle to the impact of the point
        let angle = rng.gen::<f64>() * 2.0 * std::f64::consts::PI;

        for point in &points {
            if distance(&candidate, point) <= delta {
                too_close = true;
                inhibition_count += 1;
            }
        }

        if !too_close || rng.gen::<f64>() <= c.powi(inhibition_count as i32) {
            points.push(candidate);
        }

        iterations += 1;
        if iterations >= i_max {
            println!("Warning: Maximum number of iterations reached. {}/{} points were generated.", points.len(), n);
            println!("> Equivalent fracture intensity: {}", points.len() as f64 / (width * height));
            break;
        }
    }

    points
}