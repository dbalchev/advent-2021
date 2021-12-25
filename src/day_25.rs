use std::io::BufRead;

use itertools::Itertools;
use ndarray::{stack, Array1, Array2, Axis, Zip};

use crate::common::MyResult;

fn move_direction(old_state: &Array2<u8>, axis: usize, moving_symbol: u8) -> Array2<u8> {
    return Zip::indexed(old_state).map_collect(|ind, &current| {
        let ind = [ind.0, ind.1];
        let n = old_state.shape()[axis];
        let mut prev_ind = ind;
        prev_ind[axis] = (prev_ind[axis] + n - 1) % n;
        let prev = old_state[prev_ind];
        let mut next_ind = ind;
        next_ind[axis] = (next_ind[axis] + 1) % n;
        let next = old_state[next_ind];

        if current == b'.' && prev == moving_symbol {
            moving_symbol
        } else if current == moving_symbol && next == b'.' {
            b'.'
        } else {
            current
        }
    });
}

pub fn run_me(reader: impl BufRead) -> MyResult<()> {
    let initial_state = stack(
        Axis(0),
        &(reader
            .lines()
            .map_ok(|line| Array1::from_vec(line.into_bytes()))
            .collect::<Result<Vec<_>, _>>()?
            .iter()
            .map(Array1::view)
            .collect_vec()),
    )?;
    // println!("{:?}", initial_state.mapv(|b| b as char));
    // println!("{:?}", move_direction(&initial_state, 0, b'v').mapv(|b| b as char));
    let mut current_state = initial_state;
    let mut n_steps = -1;
    for step in 1..1_000 {
        let next_state = move_direction(&current_state, 1, b'>');
        let next_state = move_direction(&next_state, 0, b'v');
        if next_state == current_state {
            n_steps = step;
            break;
        } else {
            current_state = next_state;
        }
    }
    println!("Task 1: {}", n_steps);
    Ok(())
}
