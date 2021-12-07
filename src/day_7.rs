use crate::MyResult;
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
    let min_pos = *crab_positions.iter().min().unwrap();
    let max_pos = *crab_positions.iter().max().unwrap();

    let best_cost: i32 = (min_pos..=max_pos)
        .map(|align_target| {
            crab_positions
                .iter()
                .map(|pos| (pos - align_target).abs())
                .sum()
        })
        .min()
        .unwrap();
    println!("Task 1: {}", best_cost);
    let best_cost: i32 = (min_pos..=max_pos)
        .map(|align_target| {
            crab_positions
                .iter()
                .map(|pos| (pos - align_target).abs())
                .map(cost_per_step_2)
                .fold(0i32, |acc, x| acc.checked_add(x).unwrap())
        })
        .min()
        .unwrap();
    println!("Task 2: {}", best_cost);
    Ok(())
}
