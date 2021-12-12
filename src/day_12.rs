use crate::MyResult;
use std::collections::{HashMap, HashSet};
use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_line(line: &str) -> (String, String) {
    let (a, b) = line.split_once('-').unwrap();
    return (String::from(a), String::from(b));
}

struct State {
    current_node: String,
    visited_nodes: HashSet<String>,
    visited_twice: Option<String>,
}

fn count_visits(neighbors: &HashMap<String, Vec<String>>, allow_visited_twice: bool) -> i32 {
    let mut stack = vec![State {
        current_node: String::from("start"),
        visited_nodes: HashSet::from([String::from("start")]),
        visited_twice: None,
    }];
    let mut num_paths = 0i32;

    while let Some(State {
        current_node,
        visited_nodes,
        visited_twice,
    }) = stack.pop()
    {
        for next_node in &neighbors[&current_node] {
            let is_next_small = next_node.chars().next().unwrap().is_lowercase();
            let mut new_visited_twice = visited_twice.clone();
            if next_node == "end" {
                num_paths += 1;
                continue;
            } else if is_next_small && visited_nodes.contains(next_node) {
                if allow_visited_twice && visited_twice.is_none() && next_node != "start" {
                    new_visited_twice = Some(next_node.clone());
                } else {
                    continue;
                }
            }
            let mut new_visited = visited_nodes.clone();
            if is_next_small {
                new_visited.insert(next_node.clone());
            }
            stack.push(State {
                current_node: next_node.clone(),
                visited_nodes: new_visited,
                visited_twice: new_visited_twice,
            });
        }
    }
    return num_paths;
}

pub fn run_me() -> MyResult<()> {
    let mut neighbors: HashMap<String, Vec<String>> = HashMap::new();
    BufReader::new(File::open(args().nth(1).unwrap())?)
        .lines()
        .map(|x| parse_line(&x.unwrap()))
        .for_each(|(a, b)| {
            for (x, y) in [(&a, &b), (&b, &a)] {
                neighbors.entry(x.clone()).or_default().push(y.clone());
            }
        });
    // println!("{:?}", neighbors);

    println!("Task 1: {}", count_visits(&neighbors, false));
    println!("Task 2: {}", count_visits(&neighbors, true));
    Ok(())
}
