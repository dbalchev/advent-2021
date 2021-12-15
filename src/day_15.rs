use crate::MyResult;
use ndarray::Array1;
use ndarray::Array2;
use ndarray::Axis;
use ndarray::{concatenate, stack};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::BufRead;

fn explore(risk_level: &Array2<i32>) -> i32 {
    let mi = risk_level.shape()[0] as i32;
    let mj = risk_level.shape()[1] as i32;
    let mut min_risk = Array2::from_elem((risk_level.shape()[0], risk_level.shape()[1]), i32::MAX);
    let mut heap = BinaryHeap::new();

    min_risk[[0, 0]] = 0;
    heap.push(Reverse((0, (0i32, 0i32))));

    while let Some(Reverse((current_risk, (i, j)))) = heap.pop() {
        if current_risk > min_risk[[i as usize, j as usize]] {
            continue;
        }
        if i + 1 == mi && j + 1 == mj {
            break;
        }
        for (ni, nj) in [(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)] {
            if !(0..mi).contains(&ni) || !(0..mj).contains(&nj) {
                continue;
            }
            let candidate_risk = current_risk + risk_level[[ni as usize, nj as usize]];
            let mr = &mut min_risk[[ni as usize, nj as usize]];
            if candidate_risk < *mr {
                *mr = candidate_risk;
                heap.push(Reverse((candidate_risk, (ni, nj))));
            }
        }
    }
    min_risk[[mi as usize - 1, mj as usize - 1]]
}

pub fn run_me(reader: impl BufRead) -> MyResult<()> {
    let risk_level_rows = reader
        .lines()
        .map(|line| {
            Ok(line?
                .chars()
                .map(|c| c as i32 - '0' as i32)
                .collect::<Array1<i32>>())
        })
        .collect::<MyResult<Vec<Array1<i32>>>>()?;
    let risk_level = stack(
        Axis(0),
        &risk_level_rows.iter().map(Array1::view).collect::<Vec<_>>(),
    )?;
    // println!("{:?}", risk_level);

    println!("Task 1: {}", explore(&risk_level));

    let risk_level = concatenate![
        Axis(0),
        risk_level.clone(),
        risk_level.clone() + 1,
        risk_level.clone() + 2,
        risk_level.clone() + 3,
        risk_level.clone() + 4
    ];
    let mut risk_level = concatenate![
        Axis(1),
        risk_level.clone(),
        risk_level.clone() + 1,
        risk_level.clone() + 2,
        risk_level.clone() + 3,
        risk_level.clone() + 4
    ];
    risk_level.iter_mut().for_each(|x| {
        *x = 1 + (*x - 1) % 9;
    });

    // println!("{:?}", risk_level);

    println!("Task 2: {}", explore(&risk_level));

    Ok(())
}
