use crate::{common::GeneralError, MyResult};
use ndarray::arr1;
use ndarray::Array2;
use ndarray::Axis;
use ndarray::Slice;
use ndarray::{s, stack};
use std::collections::HashSet;
use std::io::BufRead;
use std::str::FromStr;

struct Fold {
    axis: i32,
    place: i32,
}

fn count_dots_after_folds(mut dot_coords: Array2<i32>, folds: &[Fold]) -> i32 {
    for Fold { axis, place } in folds {
        for x in dot_coords.index_axis_mut(Axis(1), *axis as usize) {
            if *x > *place {
                *x = place - (*x - place);
            }
        }
    }
    dot_coords
        .axis_iter(Axis(0))
        .map(|x| x.to_vec())
        .collect::<HashSet<Vec<i32>>>()
        .len() as i32
}

pub fn run_me(reader: impl BufRead) -> MyResult<()> {
    let lines = reader.lines();
    let mut rows = Vec::new();
    let mut reading_dots = true;
    let mut folds = Vec::new();
    for line in lines {
        let line = line?;
        if line.len() == 0 {
            reading_dots = false;
            continue;
        }
        if reading_dots {
            let (x, y) = line
                .split_once(',')
                .ok_or(GeneralError(format!("Cannot split {}", line)))?;
            let x = i32::from_str(x)?;
            let y = i32::from_str(y)?;
            rows.push(arr1(&[x, y]));
        } else {
            let (message, place) = line
                .split_once("=")
                .ok_or(GeneralError(format!("Cannot split fold {}", line)))?;
            let axis = match message
                .chars()
                .last()
                .ok_or(GeneralError(format!("Message must be non-empty")))?
            {
                'x' => 0,
                'y' => 1,
                a @ _ => {
                    return Err(Box::new(GeneralError(format!(
                        "Last message char is {}",
                        a
                    ))))
                }
            };
            folds.push(Fold {
                axis,
                place: i32::from_str(place)?,
            });
        }
    }
    let dot_coords = stack(Axis(0), &rows.iter().map(|x| x.view()).collect::<Vec<_>>())?;
    // println!("{:?}", dot_coords);
    println!(
        "Task 1: {}",
        count_dots_after_folds(dot_coords, &folds[..1])
    );
    Ok(())
}
