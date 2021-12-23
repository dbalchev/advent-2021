use std::{
    cmp::Reverse,
    collections::{hash_map::Entry, BinaryHeap, HashMap, HashSet, VecDeque},
    io::BufRead,
};

use itertools::Itertools;

use crate::common::MyResult;

fn cost_multiplier(x: u8) -> i32 {
    match x {
        b'A' => 1,
        b'B' => 10,
        b'C' => 100,
        b'D' => 1000,
        _ => unreachable!("{}", x as char),
    }
}

fn generate_next_steps(maze: Vec<Vec<u8>>, target: &Vec<Vec<u8>>) -> Vec<(i32, Vec<Vec<u8>>)> {
    let mut result = Vec::new();

    for i in 0..maze.len() {
        for j in 0..maze[i].len() {
            let current_char = maze[i][j];
            if !(b'A'..=b'D').contains(&current_char) {
                continue;
            }

            let mut visited = HashSet::from([(i as i32, j as i32)]);
            let mut queue = VecDeque::from([(i as i32, j as i32, 0)]);
            while let Some((ci, cj, moves)) = queue.pop_back() {
                // println!("{} {} {} {}", i, j, ci, cj);
                for (di, dj) in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
                    let ni = ci + di;
                    let nj = cj + dj;

                    if maze[ni as usize][nj as usize] != b'.' || visited.contains(&(ni, nj)) {
                        continue;
                    }
                    let not_in_hallway = i > 1;
                    let going_to_hallway = ni == 1;
                    let not_going_at_door = target[ni as usize + 1][nj as usize] == b'#';
                    let going_to_own_room = target[ni as usize][nj as usize] == current_char
                        && ((ni + 1) as usize..maze.len() - 1)
                            .all(|x| maze[x][nj as usize] == target[x][nj as usize]);
                    if going_to_own_room
                        || (not_in_hallway
                            && going_to_hallway
                            && not_going_at_door
                            && (target[i][j] != current_char
                                || ((i..maze.len() - 1).any(|x| maze[x][j] != target[x][j]))))
                    {
                        let mut new_maze = maze.clone();
                        new_maze[i][j] = b'.';
                        new_maze[ni as usize][nj as usize] = current_char;
                        let cost = cost_multiplier(current_char) * (moves + 1);
                        result.push((cost, new_maze));
                    }
                    visited.insert((ni, nj));
                    queue.push_front((ni, nj, moves + 1));
                }
            }
        }
    }

    return result;
}

fn make_target_maze() -> Vec<Vec<u8>> {
    return [
        "#############",
        "#...........#",
        "###A#B#C#D###",
        "  #A#B#C#D#  ",
        "  #########  ",
    ]
    .into_iter()
    .map(|x| String::from(x).into_bytes())
    .collect_vec();
}

fn cost_lower_bound(maze: &Vec<Vec<u8>>) -> i32 {
    return 0;
    // let mut sum = 0;
    // for i in 0..maze.len() {
    //     for j in 0..maze.len() {
    //         let c = maze[i][j];
    //         let target_j = match c {
    //             b'A' => 3,
    //             b'B' => 5,
    //             b'C' => 7,
    //             b'D' => 9,
    //             _ => continue,
    //         };
    //         if i > 1 && j == target_j {
    //             continue;
    //         }
    //         let delta_j = (j as i32 - target_j as i32).abs();
    //         sum += cost_multiplier(c) * (delta_j + (i as i32 - 1) + 1);
    //     }
    // }
    // return sum;
}

fn solve(starting_maze: Vec<Vec<u8>>, target_maze: Vec<Vec<u8>>) -> Option<i32> {
    let mut queue = BinaryHeap::from([((
        Reverse((cost_lower_bound(&starting_maze), 0)),
        starting_maze,
    ))]);
    let mut best_cost = None;
    let mut min_cost = HashMap::new();
    let mut last_printed = 0;
    while let Some((Reverse((heuristic_cost, actual_cost)), current_maze)) = queue.pop() {
        if heuristic_cost > last_printed + 1_000 {
            println!("reached {}", heuristic_cost);
            last_printed = 1_000 + heuristic_cost;
        }
        // println!(
        //     "{}",
        //     current_maze
        //         .clone()
        //         .into_iter()
        //         .map(|x| String::from_utf8(x).unwrap())
        //         .join("\n")
        // );
        // println!("{} {}", heuristic_cost, actual_cost);
        if let Entry::Occupied(oe) = min_cost.entry(current_maze.clone()) {
            if *oe.get() < heuristic_cost {
                continue;
            }
        }
        if current_maze == target_maze {
            best_cost = Some(actual_cost);
            break;
        }
        for (move_cost, next_maze) in generate_next_steps(current_maze, &target_maze) {
            assert!(move_cost > 0, "{}", move_cost);
            let total_actual_cost = actual_cost + move_cost;
            let total_predicted_cost = cost_lower_bound(&next_maze) + total_actual_cost;
            let should_push = match min_cost.entry(next_maze.clone()) {
                Entry::Occupied(mut oe) => {
                    if *oe.get() > total_predicted_cost {
                        *oe.get_mut() = total_predicted_cost;
                        true
                    } else {
                        false
                    }
                }
                Entry::Vacant(ve) => {
                    ve.insert(total_predicted_cost);
                    true
                }
            };
            if should_push {
                // println!("{}", next_maze.clone().into_iter().map(|x| String::from_utf8(x).unwrap()).join("\n"));
                // println!("{}", total_cost);
                queue.push((
                    Reverse((total_predicted_cost, total_actual_cost)),
                    next_maze,
                ));
            }
        }
    }
    best_cost
}

pub fn run_me(reader: impl BufRead) -> MyResult<()> {
    let mut starting_maze = reader
        .lines()
        .map_ok(String::into_bytes)
        .collect::<Result<Vec<_>, _>>()?;
    let mut target_maze = make_target_maze();
    println!(
        "Task 1 {:?}",
        solve(starting_maze.clone(), target_maze.clone())
    );

    starting_maze.insert(3, String::from("  #D#C#B#A#  ").into_bytes());
    starting_maze.insert(4, String::from("  #D#B#A#C#  ").into_bytes());
    target_maze.insert(3, String::from("  #A#B#C#D#  ").into_bytes());
    target_maze.insert(4, String::from("  #A#B#C#D#  ").into_bytes());

    println!(
        "Task 2 {:?}",
        solve(starting_maze.clone(), target_maze.clone())
    );

    // println!("{}", target_maze.clone().into_iter().map(|x| String::from_utf8(x).unwrap()).join("\n"));
    // println!("{}", starting_maze.clone().into_iter().map(|x| String::from_utf8(x).unwrap()).join("\n"));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step_1() {
        let start_maze = [
            "#############",
            "#...........#",
            "###B#C#B#D###",
            "  #A#D#C#A#  ",
            "  #########  ",
        ]
        .into_iter()
        .map(|x| String::from(x).into_bytes())
        .collect_vec();
        let required_maze = [
            "#############",
            "#...B.......#",
            "###B#C#.#D###",
            "  #A#D#C#A#  ",
            "  #########  ",
        ]
        .into_iter()
        .map(|x| String::from(x).into_bytes())
        .collect_vec();
        assert!(generate_next_steps(start_maze, &make_target_maze()).contains(&(40, required_maze)));
    }
    #[test]
    fn test_step_2() {
        let start_maze = [
            "#############",
            "#...B.......#",
            "###B#C#.#D###",
            "  #A#D#C#A#  ",
            "  #########  ",
        ]
        .into_iter()
        .map(|x| String::from(x).into_bytes())
        .collect_vec();
        let required_maze = [
            "#############",
            "#...B.......#",
            "###B#.#C#D###",
            "  #A#D#C#A#  ",
            "  #########  ",
        ]
        .into_iter()
        .map(|x| String::from(x).into_bytes())
        .collect_vec();
        let next_steps = generate_next_steps(start_maze, &make_target_maze());
        assert!(next_steps.contains(&(400, required_maze)));
    }
    #[test]
    fn test_step_3() {
        let start_maze = [
            "#############",
            "#.....D.....#",
            "###.#B#C#D###",
            "  #A#B#C#A#  ",
            "  #########  ",
        ]
        .into_iter()
        .map(|x| String::from(x).into_bytes())
        .collect_vec();
        let required_maze = [
            "#############",
            "#.....D.D...#",
            "###.#B#C#.###",
            "  #A#B#C#A#  ",
            "  #########  ",
        ]
        .into_iter()
        .map(|x| String::from(x).into_bytes())
        .collect_vec();
        let next_steps = generate_next_steps(start_maze, &make_target_maze());
        // for (cost, current_maze) in &next_steps {
        //     println!(
        //         "{}",
        //         current_maze
        //             .clone()
        //             .into_iter()
        //             .map(|x| String::from_utf8(x).unwrap())
        //             .join("\n")
        //     );
        //     println!("{}", cost);
        // }
        assert!(next_steps.contains(&(2000, required_maze)));
    }
}
