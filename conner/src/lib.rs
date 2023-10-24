use rayon::prelude::*;
use std::time::Instant;
use indicatif::{ProgressBar, ProgressStyle};
use pyo3::prelude::*;

#[pyfunction]
fn check_points_seq(points: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut matchings = vec![vec![0_i32;0]; points.len()];
    // check if points is matched already (0 = not matched, 1 = matched)
    let mut matched_points = vec![0_i32; points.len()];
    let p_len = points.len();


    let pb = ProgressBar::new(p_len as u64);
    pb.set_style(ProgressStyle::with_template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
    )
        .unwrap()
        .progress_chars("_/^")
    );


    for i in 0..p_len{
        if i % 1000 == 0 {
            // pb.set_message(&format!("{} / {}", i, p_len));
            pb.inc(1000);
        }

        if matched_points[i] == 1 {
            continue;
        }
        let mut i_matchings = vec![i as i32;1];

        for j in i..points.len(){
            if matched_points[j] == 1 {
                continue;
            }

            let p = &points[i];
            let p2 = &points[j];

            if p[0] == p2[0] && p[1] == p2[1] {
                matched_points[i] = 1;
                matched_points[j] = 1;
                i_matchings.push(j as i32)
            }
        }
        matchings[i] = i_matchings;
    }

    matchings

}

#[pyfunction]
fn check_points(points: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    println!("Running rust kernel");

    let now = Instant::now();
    let p_len = points.len();

    let num_chunks = rayon::current_num_threads() * 5;
    let chunk_size = p_len / num_chunks;
    println!("Running on {} chunks", num_chunks);

    let pb = ProgressBar::new(p_len as u64);
    pb.set_style(ProgressStyle::with_template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
    )
        .unwrap()
        .progress_chars("##-")
    );

    let mut matching_points = vec![vec![0;0]; p_len];
    matching_points
        .par_chunks_mut(chunk_size)
        .enumerate()
        .panic_fuse()
        .for_each(|(chunk_index, chunk)| {
            let chunk_id0 = chunk_index * chunk_size;

            // iterate from chunk start to chunk end (points)
            for (i, point_matches) in chunk.iter_mut().enumerate() {
                pb.inc(1);
                // find current index (convention divides array by rows)
                let ix = chunk_id0 + i;


                // overwrite chunk with result
                *point_matches = check_point(&points, ix);

                // // calculate the new location
                // for r in result {
                //     point_matches.push(r);
                // }
            }

            // for c in chunk {
            //     pb.inc(1);
            //     let matches = check_point(&points, *c);
            //     if matches.len() > 1 {
            //         for m in matches{
            //             matching_points.push([*c, m]);
            //         }
            //     }
            // }
        });

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    matching_points
}

fn check_point(points: &[Vec<i32>], index: usize) -> Vec<i32> {
    let mut matches = vec![0_i32;0];
    let p_len = points.len();
    for i in index+1..p_len {
        if points[index][0] == points[i][0] && points[index][1] == points[i][1] {
            matches.push(i as i32);
        }
    }
    matches
}

#[pymodule]
fn conner(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(check_points, m)?)?;
    m.add_function(wrap_pyfunction!(check_points_seq, m)?)?;
    Ok(())
}

#[test]
fn test_points(){
    let mut points = vec![vec![0;2]; 743977];

    // create random numbers in array
    for p in points.iter_mut() {
        // create random point
        p[0] = rand::random::<i32>();
        p[1] = rand::random::<i32>();
    }

    check_points(points);
}