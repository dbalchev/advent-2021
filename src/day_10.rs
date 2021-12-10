use crate::MyResult;
use std::env::args;
use std::fs::File;
use std::io::BufReader;
use std::io::{BufRead, Result as IOResult};

#[derive(Clone, Copy)]
struct ClosingInfo {
    closing_char: u8,
    completion_score: i64,
}

#[derive(Clone, Copy)]
enum CharType {
    Other,
    Opening(ClosingInfo),
    Closing { error_score: i32 },
}

enum LineResult {
    Corrupted(i32),
    Incomplete(i64),
}

const CHAR_MAP: [CharType; 256] = {
    let mut cm = [CharType::Other; 256];
    cm[b'(' as usize] = CharType::Opening(ClosingInfo {
        closing_char: b')',
        completion_score: 1,
    });
    cm[b')' as usize] = CharType::Closing { error_score: 3 };

    cm[b'[' as usize] = CharType::Opening(ClosingInfo {
        closing_char: b']',
        completion_score: 2,
    });
    cm[b']' as usize] = CharType::Closing { error_score: 57 };

    cm[b'{' as usize] = CharType::Opening(ClosingInfo {
        closing_char: b'}',
        completion_score: 3,
    });
    cm[b'}' as usize] = CharType::Closing { error_score: 1197 };

    cm[b'<' as usize] = CharType::Opening(ClosingInfo {
        closing_char: b'>',
        completion_score: 4,
    });
    cm[b'>' as usize] = CharType::Closing { error_score: 25137 };
    cm
};

fn score_line(line: &str) -> LineResult {
    let mut opened_stack = Vec::<ClosingInfo>::new();
    for b_char in line.bytes() {
        match CHAR_MAP[b_char as usize] {
            CharType::Other => panic!(),
            CharType::Opening(ci) => opened_stack.push(ci),
            CharType::Closing { error_score } => {
                let top_of_stack = opened_stack.pop().unwrap();
                if b_char != top_of_stack.closing_char {
                    return LineResult::Corrupted(error_score);
                }
            }
        }
    }
    opened_stack.reverse();
    let mut total_completion_score: i64 = 0;
    for ClosingInfo {
        completion_score, ..
    } in opened_stack
    {
        total_completion_score = total_completion_score
            .checked_mul(5)
            .unwrap()
            .checked_add(completion_score)
            .unwrap();
    }

    return LineResult::Incomplete(total_completion_score);
}

pub fn run_me() -> MyResult<()> {
    let lines: Vec<String> = BufReader::new(File::open(args().nth(1).unwrap())?)
        .lines()
        .collect::<IOResult<Vec<String>>>()?;
    let task_1_scores: i32 = lines
        .iter()
        .filter_map(|x| match score_line(x) {
            LineResult::Corrupted(error_score) => Some(error_score),
            _ => None,
        })
        .sum();

    println!("Task 1: {}", task_1_scores);

    let mut task_2_scores: Vec<i64> = lines
        .iter()
        .filter_map(|x| match score_line(x) {
            LineResult::Incomplete(error_score) => Some(error_score),
            _ => None,
        })
        .collect();
    task_2_scores.sort();
    println!("Task 2: {}", task_2_scores[task_2_scores.len() / 2]);

    Ok(())
}
