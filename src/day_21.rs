use std::io::BufRead;

use ndarray::{s, ArrayD, Axis, IxDyn, Slice};

use crate::common::MyResult;
use std::str::FromStr;

const LINE_PREFIX_1: &str = "Player 1 starting position: ";
const LINE_PREFIX_2: &str = "Player 2 starting position: ";

const ROLLS_AND_FREQS: &[(usize, i128)] = &[
    (3, 1), // 1,1,1
    (4, 3), // 1,1,2 1,2,1 2,1,1
    (5, 6), // 1,1,3 1,2,2
    (6, 7), // 1,2,3 2,2,2
    (7, 6),
    (8, 3),
    (9, 1),
];

pub fn run_me(reader: impl BufRead) -> MyResult<()> {
    let mut lines = reader.lines();
    let line_1 = lines.next().ok_or("line 1 is missing")??;
    let line_2 = lines.next().ok_or("line 2 is missing")??;
    for (line, prefix) in [(&line_1, LINE_PREFIX_1), (&line_2, LINE_PREFIX_2)] {
        if !line.starts_with(prefix) {
            Err(format!("{} doesn't start with {}", line, prefix))?;
        }
    }
    let player_1_pos = usize::from_str(&line_1[LINE_PREFIX_1.len()..])?;
    let player_2_pos = usize::from_str(&line_2[LINE_PREFIX_2.len()..])?;

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

    let mut memo = ArrayD::from_elem(IxDyn(&[2, 11, 11, 31, 31]), 0i128);
    memo[[0, player_1_pos, player_2_pos, 0, 0]] = 1;
    for score_sum in 0..=40 {
        for player_1_score in 0..=score_sum {
            let player_2_score = score_sum - player_1_score;
            if player_1_score >= 21 || player_2_score >= 21 {
                continue;
            }
            for current_player in [0, 1] {
                for player_1_pos in 1..=10 {
                    for player_2_pos in 1..=10 {
                        for (roll, freq) in ROLLS_AND_FREQS {
                            let mut positions = [player_1_pos, player_2_pos];
                            let mut scores = [player_1_score, player_2_score];
                            let p = &mut positions[current_player];
                            // println!("{} {}", *p, moves);
                            *p = (*p + roll - 1) % 10 + 1;
                            scores[current_player] += *p;
                            memo[[
                                1 - current_player,
                                positions[0],
                                positions[1],
                                scores[0],
                                scores[1],
                            ]] += freq
                                * memo[[
                                    current_player,
                                    player_1_pos,
                                    player_2_pos,
                                    player_1_score,
                                    player_2_score,
                                ]];
                        }
                    }
                }
            }
        }
    }
    let player_1_wins = memo.slice_axis(Axis(3), Slice::from(21..)).sum();
    let player_2_wins = memo.slice_axis(Axis(4), Slice::from(21..)).sum();
    println!("{} {}", player_1_wins, player_2_wins);
    println!("Task 2: {}", player_1_wins.max(player_2_wins));
    Ok(())
}
