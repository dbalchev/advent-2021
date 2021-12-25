use std::io::BufRead;

use itertools::Itertools;
use ndarray::{Array1, Zip};

use std::str::FromStr;

use crate::common::MyResult;

pub fn run_me(reader: impl BufRead) -> MyResult<()> {
    let measurements = Array1::from_vec(
        reader
            .lines()
            .map(|line| Ok(i32::from_str(&(line?))?))
            .collect::<MyResult<Vec<_>>>()?,
    );

    println!(
        "Task 1: {}",
        measurements
            .windows([2])
            .into_iter()
            .map(|w| (w[[0]] < w[[1]]) as i32)
            .sum::<i32>()
    );

    let measurements_3 = Zip::from(measurements.windows([3])).map_collect(|w| w.sum());

    println!(
        "Task 2: {}",
        measurements_3
            .windows([2])
            .into_iter()
            .map(|w| (w[[0]] < w[[1]]) as i32)
            .sum::<i32>()
    );

    Ok(())
}
