use crate::MyResult;
use itertools::Itertools;
use ndarray::Array2;
use ndarray::Axis;
use ndarray::Zip;
use ndarray::{concatenate, s, stack, Array1};
use std::collections::HashMap;
use std::io::BufRead;

fn label_area(is_foreground: &Array2<bool>) -> Array2<i32> {
    let mut label = is_foreground.mapv(|_| 0);
    let mut next_label = 1;

    Zip::indexed(is_foreground).for_each(|(i, j), &is_fg| {
        if is_fg && label[[i, j]] == 0 {
            let mut stack = vec![(i, j)];
            let current_label = next_label;
            next_label += 1;
            label[[i, j]] = current_label;
            while let Some((ci, cj)) = stack.pop() {
                for (di, dj) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                    let ni = (ci as i32 + di) as usize;
                    let nj = (cj as i32 + dj) as usize;

                    if is_foreground[[ni, nj]] && label[[ni, nj]] == 0 {
                        label[[ni, nj]] = current_label;
                        stack.push((ni, nj));
                    }
                }
            }
        }
    });

    label
}

pub fn run_me(reader: impl BufRead) -> MyResult<()> {
    let map_rows = reader
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

    let labels = label_area(&padded_map.mapv(|x| x < 9));
    let area_sizes = {
        let mut freq = labels
            .iter()
            .filter(|&&l| l != 0)
            .fold(HashMap::new(), |mut counter, &l| {
                *(counter.entry(l).or_insert(0)) += 1;
                counter
            })
            .values()
            .cloned()
            .collect_vec();
        freq.sort();
        freq
    };
    println!(
        "Task 2: {}",
        area_sizes[area_sizes.len() - 3..].iter().product::<i32>()
    );
    // println!("{:?}", padded_map);
    Ok(())
}
