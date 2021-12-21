use std::io::BufRead;

use crate::common::MyResult;
use std::str::FromStr;

const LINE_PREFIX_1: &str = "Player 1 starting position: ";
const LINE_PREFIX_2: &str = "Player 2 starting position: ";

pub fn run_me(reader: impl BufRead) -> MyResult<()> {
    let mut lines = reader.lines();
    let line_1 = lines.next().ok_or("line 1 is missing")??;
    let line_2 = lines.next().ok_or("line 2 is missing")??;
    for (line, prefix) in [(&line_1, LINE_PREFIX_1), (&line_2, LINE_PREFIX_2)] {
        if !line.starts_with(prefix) {
            Err(format!("{} doesn't start with {}", line, prefix))?;
        }
    }
    let player_1_pos = i32::from_str(&line_1[LINE_PREFIX_1.len()..])?;
    let player_2_pos = i32::from_str(&line_2[LINE_PREFIX_2.len()..])?;

    println!("{} {}", player_1_pos, player_2_pos);

    let mut positions = [player_1_pos, player_2_pos];
    let mut points = [0, 0];
    let mut n_rolls = 0;
    let mut current_player = 0;
    let mut current_dice_value = 1;

    let mut roll = || {
        n_rolls += 1;
        let r = current_dice_value;
        current_dice_value = 1 + current_dice_value % 1_000;
        r
    };

    while points.iter().max().unwrap() < &1_000 {
        let moves = roll() + roll() + roll();
        let p = &mut positions[current_player];
        // println!("{} {}", *p, moves);
        *p = (*p + moves - 1) % 10 + 1;
        points[current_player] += *p;
        // println!("score {} {} {}", moves, *p, points[current_player]);
        current_player = 1 - current_player;
    }

    println!("Task 1: {}", points.iter().min().unwrap() * n_rolls);
    Ok(())
}
