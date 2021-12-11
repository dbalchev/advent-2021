use crate::MyResult;
use ndarray::stack;
use ndarray::Array1;
use ndarray::Array2;
use ndarray::Axis;
use std::env::args;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

const ADJ_DELTA: &[(i32, i32)] = &[
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn step(energy_levels: &mut Array2<i32>) -> i32 {
    *energy_levels += 1;
    let mut flash_locations: Vec<(i32, i32)> = energy_levels
        .indexed_iter()
        .filter_map(|((y, x), a)| {
            if *a > 9 {
                Some((y as i32, x as i32))
            } else {
                None
            }
        })
        .collect();
    while let Some((fy, fx)) = flash_locations.pop() {
        for (dy, dx) in ADJ_DELTA {
            let y = fy + dy;
            let x = fx + dx;
            if !(0..energy_levels.shape()[0] as i32).contains(&y)
                || !(0..energy_levels.shape()[1] as i32).contains(&x)
                || energy_levels[(y as usize, x as usize)] > 9
            {
                continue;
            }
            let c = &mut energy_levels[(y as usize, x as usize)];
            *c += 1;
            if *c > 9 {
                flash_locations.push((y, x));
            }
        }
    }

    let total_flashes = energy_levels
        .iter_mut()
        .map(|x| {
            if *x > 9 {
                *x = 0;
                1
            } else {
                0
            }
        })
        .sum();
    total_flashes
}

pub fn run_me() -> MyResult<()> {
    let energy_levels_rows = BufReader::new(File::open(args().nth(1).unwrap())?)
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c as i32 - '0' as i32)
                .collect::<Array1<i32>>()
        })
        .collect::<Vec<Array1<i32>>>();
    let initial_energy_levels = stack(
        Axis(0),
        &energy_levels_rows
            .iter()
            .map(Array1::view)
            .collect::<Vec<_>>(),
    )?;
    let mut energy_levels = initial_energy_levels.clone();
    let mut total_flashes = 0;
    for _ in 0..100 {
        total_flashes += step(&mut energy_levels);
    }
    println!("Task 1: {}", total_flashes);

    let mut energy_levels = initial_energy_levels.clone();
    let first_sync_step = (1..1_000).find(|step_no| {
        step(&mut energy_levels);
        energy_levels.iter().all(|&x| x == 0)
    });
    println!("Task 2: {:?}", first_sync_step);
    Ok(())
}
