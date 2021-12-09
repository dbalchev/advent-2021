use crate::MyResult;
use ndarray::Array2;
use ndarray::Axis;
use ndarray::Zip;
use ndarray::{concatenate, s, stack, Array1};
use std::env::args;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn run_me() -> MyResult<()> {
    let map_rows = BufReader::new(File::open(args().nth(1).unwrap())?)
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c as i32 - '0' as i32)
                .collect::<Array1<i32>>()
        })
        .collect::<Vec<Array1<i32>>>();
    let map = stack(
        Axis(0),
        &map_rows.iter().map(Array1::view).collect::<Vec<_>>(),
    )?;
    let row_padding = Array2::from_elem([1, map.shape()[1] + 2], 10);
    let col_padding = Array2::from_elem([map.shape()[0], 1], 10);
    let padded_map = concatenate!(
        Axis(0),
        row_padding,
        concatenate!(Axis(1), col_padding, map, col_padding),
        row_padding,
    );
    let n_rows = padded_map.shape()[0];
    let n_cols = padded_map.shape()[1];
    let solution_1: i32 = padded_map
        .windows([3, 3])
        .into_iter()
        .filter(|w| {
            let mid = w[[1, 1]];
            return mid < w[[0, 1]] && mid < w[[1, 0]] && mid < w[[1, 2]] && mid < w[[2, 1]];
        })
        .map(|w| w[[1, 1]] + 1)
        .sum();
    println!("Task 1: {}", solution_1);
    // println!("{:?}", padded_map);
    Ok(())
}
