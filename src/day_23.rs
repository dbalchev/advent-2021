use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet, VecDeque},
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
                    if target[ni as usize][nj as usize] == current_char
                        || ([2, 3].contains(&i)
                            && ni == 1
                            && (target[ni as usize + 1][nj as usize] == b'#')
                            && (target[i][j] != current_char
                                || (i == 2 && maze[i + 1][j] != target[i + 1][j])))
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

pub fn run_me(reader: impl BufRead) -> MyResult<()> {
    let starting_maze = reader
        .lines()
        .map_ok(String::into_bytes)
        .collect::<Result<Vec<_>, _>>()?;
    let target_maze = make_target_maze();
    let mut queue = BinaryHeap::from([((Reverse(0), starting_maze))]);

    let mut best_cost = None;

    let mut visited = HashSet::new();

    while let Some((Reverse(cost), current_maze)) = queue.pop() {
        // println!(
        //     "{}",
        //     current_maze
        //         .clone()
        //         .into_iter()
        //         .map(|x| String::from_utf8(x).unwrap())
        //         .join("\n")
        // );
        // println!("{}", cost);
        if visited.contains(&current_maze) {
            continue;
        }
        visited.insert(current_maze.clone());
        if current_maze == target_maze {
            best_cost = Some(cost);
            break;
        }
        for (move_cost, next_maze) in generate_next_steps(current_maze, &target_maze) {
            assert!(move_cost > 0, "{}", move_cost);
            let total_cost = cost + move_cost;
            if !visited.contains(&next_maze) {
                // println!("{}", next_maze.clone().into_iter().map(|x| String::from_utf8(x).unwrap()).join("\n"));
                // println!("{}", total_cost);
                queue.push((Reverse(total_cost), next_maze));
            }
        }
    }
    println!("Task 1 {:?}", best_cost);
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
