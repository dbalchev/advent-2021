use crate::MyResult;
use ndarray::{arr1, s, Array, Axis, NewAxis};
use std::env::args;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

fn cost_per_step_2(delta: i32) -> i32 {
    delta * (delta + 1) / 2
}

pub fn run_me() -> MyResult<()> {
    let crab_positions: Vec<i32> = BufReader::new(File::open(args().nth(1).unwrap())?)
        .lines()
        .next()
        .unwrap()?
        .split(",")
        .map(|x| i32::from_str(x).unwrap())
        .collect();
    let crab_positions = arr1(&crab_positions);
    let min_pos = *crab_positions.iter().min().unwrap();
    let max_pos = *crab_positions.iter().max().unwrap();
    let pos_range = Array::from_iter(min_pos..=max_pos);

    let deltas = (&crab_positions.slice(s![NewAxis, 0..]) - &pos_range.slice(s![0.., NewAxis]))
        .mapv(|x| x.abs());
    let delta_sum = deltas.sum_axis(Axis(1));
    let best_cost_1 = delta_sum.iter().min().unwrap();
    println!("Task 1: {}", best_cost_1);
    let delta_sum = deltas.mapv(cost_per_step_2).sum_axis(Axis(1));
    let best_cost_2 = delta_sum.iter().min().unwrap();
    println!("Task 2: {}", best_cost_2);
    Ok(())
}
