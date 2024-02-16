use pyo3::prelude::*;
use rayon::prelude::*;
use std::f64::consts::PI;


#[cfg(test)]
use rand::Rng;
#[cfg(test)]
use plotters::prelude::*;

mod spatials;
mod csproc;

use crate::spatials::csstraussproc;

use crate::csproc::poisson;
use crate::csproc::csstraussproc2;
use crate::csproc::csstraussproc_rhciter;
use crate::csproc::bohmann_process;


fn create_circle(
    point: &[f64; 2],
    r: f64,
) -> Vec<[f64; 2]>
{
    // create a circle by creating a set of points
    let n = 100;
    let mut points = Vec::with_capacity(n);
    for i in 0..n {
        let angle = 2.0 * PI * (i as f64) / (n as f64);
        let x = point[0] + r * angle.cos();
        let y = point[1] + r * angle.sin();
        points.push([x, y]);
    }
    points
}

/// Calculate the weight of a circle on a rectangular plane
fn circle_weight(
    point: &[f64; 2],
    width: f64,
    height: f64,
    r: f64,
) -> f64
{
    // create a circle
    let circle = create_circle(point, r);
    let mut n_inside = 0;

    for p in &circle {
        if p[0] >= 0.0 && p[0] <= width && p[1] >= 0.0 && p[1] <= height {
            n_inside += 1;
        }
    }

    n_inside as f64 / circle.len() as f64
}

/// estimate the K-value for a set of points and a given distance
fn kest(points: &[[f64;2]], width: f64, height: f64, d: f64, use_weights: bool) -> f64 {
    let n = points.len() as f64;
    // this iterates over all points in parallel and checks for the amount of other points within the distance d
    let k_value = points.par_chunks((n / 4.0) as usize).map(|cpoints| {
        cpoints.iter().map(|&point1| {
            // calculate the weight
            let weight = if use_weights { circle_weight(&point1, width, height, d) } else { 1.0 };

            // previously, this was: points[i + 1..].iter()...
            points.iter().filter(|&&point2| {
                point1 != point2 && euclidean_distance(&point1, &point2) < d
            }).count() as f64 * weight
        }).sum::<f64>()
    }).sum::<f64>();

    let area = width * height;

    // from cskhat (Matlab)
    area * k_value / (n*(n-1.0))
    // from https://github.com/astropy/astropy/blob/main/astropy/stats/spatial.py#L232C27-L232C27
    // 2.0 * area * k_value / (n * (n - 1.0))
}

/// create a custom distance range using square
fn custom_distance(i: usize, num_intervals: usize, max_d: f64) -> f64 {
    let i_f64 = i as f64;
    let normalized_i = i_f64 / num_intervals as f64;
    // small numbers get small distance, large numbers get large distance
    max_d * normalized_i * normalized_i
}


/// calculate the estimated K function for a set of points and multiple distances ranging from 0 to max_d
fn kfun(points: &[[f64;2]], width: f64, height: f64, max_d: f64, use_weights: bool) -> Vec<[f64;2]> {
    let num_intervals = 100;
    // let log_max_d = max_d.log10();
    // let log_min_d = 0.0;
    // let log_interval = (log_max_d - log_min_d) / num_intervals as f64;

    (0..=num_intervals)
        .map(|i| {
            let d = custom_distance(i, num_intervals, max_d);
            [d, kest(points, width, height, d, use_weights)]
        })
        .collect()
}

#[pyfunction]
fn khat_test(points: Vec<[f64;2]>, width: f64, height: f64, max_d: f64, use_weights: bool) -> Vec<[f64;2]> {
    kfun(&points, width, height, max_d, use_weights)
}

#[pyfunction]
fn lhatc_test(points: Vec<[f64;2]>, width: f64, height: f64, max_d: f64, use_weights: bool) -> Vec<[f64;2]> {
    // convert points to tuples
    let mut res = kfun(&points, width, height, max_d, use_weights);
    // sqrt(k/PI) - d, from Baddeley S.207 and Dixon 2002
    (0..res.len()).for_each(|i| {
        res[i][1] = (res[i][1] / PI).sqrt() - res[i][0];
    });

    res
}

#[pyfunction]
fn lhat_test(points: Vec<[f64;2]>, width: f64, height: f64, max_d: f64, use_weights: bool) -> Vec<[f64;2]> {
    let mut res = kfun(&points, width, height, max_d, use_weights);
    // sqrt(k/PI) - d, from Baddeley S.207 and Dixon 2002
    (0..res.len()).for_each(|i| {
        res[i][1] = (res[i][1] / PI).sqrt();
    });

    res
}



fn euclidean_distance(point1: &[f64; 2], point2: &[f64; 2]) -> f64 {
    let (x1, y1) = (point1[0], point1[1]);
    let (x2, y2) = (point2[0], point2[1]);
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}



/// A Python module implemented in Rust.
#[pymodule]
fn spazial(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(khat_test, m)?)?;
    m.add_function(wrap_pyfunction!(lhatc_test, m)?)?;
    m.add_function(wrap_pyfunction!(lhat_test, m)?)?;

    m.add_function(wrap_pyfunction!(poisson, m)?)?;
    m.add_function(wrap_pyfunction!(csstraussproc, m)?)?;
    m.add_function(wrap_pyfunction!(csstraussproc2, m)?)?;
    m.add_function(wrap_pyfunction!(csstraussproc_rhciter, m)?)?;
    m.add_function(wrap_pyfunction!(bohmann_process, m)?)?;
    Ok(())
}


#[cfg(test)]
fn generate_random_points(n: usize, width: f64, height: f64) -> Vec<[f64;2]> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| [rng.gen::<f64>() * width, rng.gen::<f64>() * height]).collect()
}

#[cfg(test)]
fn plot_points(points: &[[f64;2]], file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(file_name, (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let (x_min, x_max) = points.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), &p| {
        (min.min(p[0]), max.max(p[0]))
    });

    let (y_min, y_max) = points.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), &p| {
        (min.min(p[1]), max.max(p[1]))
    });

    let mut chart = ChartBuilder::on(&root)
        .caption("Ripley's K-Funktion Test", ("sans-serif", 30).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(x_min..x_max, y_min..y_max)?;

    chart.configure_mesh().draw()?;
    chart.draw_series(PointSeries::of_element(
        points.iter().map(|p| (p[0],p[1])),
        3,
        &RED,
        &|coord, size, style| {
            EmptyElement::at(coord) + Circle::new((0, 0), size, style.filled())
        },
    ))?;

    root.present()?;
    Ok(())
}

#[cfg(test)]
fn plot_values(
    values: &[[f64;2]],
    file_name: &str,
    title: &str,
    x_title: &str,
    y_title: &str)
-> Result<(), Box<dyn std::error::Error>>
{
    let (x_min, x_max) = values.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), &p| {
        (min.min(p[0]), max.max(p[0]))
    });

    let (y_min, y_max) = values.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), &p| {
        (min.min(p[1]), max.max(p[1]))
    });
    let root = BitMapBackend::new(file_name, (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 30).into_font())
        .margin(25)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(x_min..x_max, y_min..y_max)?;

    chart.configure_mesh().x_desc(x_title).y_desc(y_title).draw()?;

    let data = Vec::from(values);
    chart.draw_series(LineSeries::new(data.iter().map(|f| (f[0],f[1])), &RED))?;

    // add labels to x and y axes

    root.present()?;
    Ok(())
}


#[test]
fn test_ripleys() -> Result<(), Box<dyn std::error::Error>>{
    let points = generate_random_points(100, 100.0, 100.0);
    let width = 10000.0; // 100x100 Fläche
    let height = 10000.0; // 100x100 Fläche
    let t = 10.0;

    let k_value = kest(&points, width, height, t, true);
    println!("Ripley's K-Funktion Wert: {}", k_value);

    plot_points(&points, "test_ripleys_points.png")?;

    Ok(())
}

#[test]
fn test_ripleys_func() -> Result<(), Box<dyn std::error::Error>>  {
    let w = 500.0;
    let h = 500.0;
    let points = generate_random_points(500, w, h);
    let area = w*h; // 100x100 Fläche
    let max_d = 50.0;
    let k_values = kfun(&points, w, h, max_d, true);
    let mut l_values = k_values.clone();

    // sqrt(k/PI) - d, from Baddeley S.207 and Dixon 2002
    (0..l_values.len()).for_each(|i| {
        l_values[i][1] = (l_values[i][1] / PI).sqrt() - l_values[i][0];
    });

    plot_values(&k_values, "test_ripleys_func_K.png", "K-Function", "d", "K")?;
    plot_values(&l_values, "test_ripleys_func_L.png", "L-Function", "d", "L")?;

    plot_points(&points, "test_ripleys_func_points.png")?;

    Ok(())
}