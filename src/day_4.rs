use std::str::FromStr;
use std::io::BufRead;
use std::fs::File;
use std::io::BufReader;
use std::io;
use crate::common::{MyResult, make_my_result};

#[derive(Debug, PartialEq)]
struct Board {
    numbers: Vec<Vec<i32>>
}

fn read(lines: &[String]) -> MyResult<(Vec<i32>, Vec<Board>)> {
    let numbers = lines[0].split(',').map(i32::from_str).map(make_my_result).collect::<MyResult<Vec<i32>>>()?;

    let mut boards = Vec::new();

    for board_start in (1..lines.len()).step_by(6) {
        assert_eq!(lines[board_start].len(), 0);
        let board_numbers: Vec<Vec<i32>> = lines[board_start + 1..board_start + 6]
            .iter()
            .map(|line| line.split_whitespace().map(i32::from_str).map(make_my_result).collect())
            .collect::<MyResult<Vec<Vec<i32>>>>()?;
        boards.push(Board{numbers: board_numbers})
    }

    return Ok((numbers, boards));
}

fn find_match(board: &Board, number: i32) -> Option<(usize, usize)> {
    for (row_no, row) in board.numbers.iter().enumerate() {
        for (col_no, cell) in row.iter().enumerate() {
            if *cell == number {
                return Some((row_no, col_no))
            }
        }
    }
    return None;
}

fn simulate(numbers: &[i32], boards: &[Board]) -> Vec<i32> {
    let mut column_matches = vec![[0i32; 5]; boards.len()];
    let mut row_matches = vec![[0i32; 5]; boards.len()];

    let mut unmarked_sum: Vec<i32> = boards.iter().map(
        |board| board.numbers.iter().map(|row| row.iter().sum::<i32>()).sum()
    ).collect();

    let mut has_won = vec![false; boards.len()];

    let mut completed_board_scores = Vec::new();

    for number in numbers {
        for (board_index, board) in boards.iter().enumerate() {
            if has_won[board_index] {
                continue;
            }
            if let Some((match_row, match_col)) = find_match(board, *number) {
                let current_row_matches = & mut row_matches[board_index][match_row];
                let current_col_matches = & mut column_matches[board_index][match_col];
                let current_unmarked_sum = & mut unmarked_sum[board_index];

                let current_number = board.numbers[match_row][match_col];
                *current_row_matches += 1;
                *current_col_matches += 1;
                *current_unmarked_sum -= current_number;
                
                if *current_row_matches == 5 || *current_col_matches == 5 {
                    has_won[board_index] = true;
                    completed_board_scores.push(current_number * (*current_unmarked_sum));
                }
            }
        }
    }
    return completed_board_scores
}

pub fn run_me() -> MyResult<()> {
    let lines = BufReader::new(File::open("inputs/day_4.txt")?)
        .lines()
        .map(make_my_result)
        .collect::<MyResult<Vec<String>>>()?;
    let (numbers, boards) = read(&lines)?;
    let completed_board_scores = simulate(&numbers, &boards);
    println!("Task 1: {}", completed_board_scores[0]);
    println!("Task 2: {}", completed_board_scores[completed_board_scores.len() - 1]);
    
    Ok(())
}

#[cfg(test)]
mod tests{
    use super::*;

    fn small_test_input() -> Vec<String> {
        [
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1",
            "",
            "22 13 17 11  0",
            "8  2 23  4 24",
            "21  9 14 16  7",
            "6 10  3 18  5",
            "1 12 20 15 19",
            "",
            "3 15  0  2 22",
            "9 18 13 17  5",
            "19  8  7 25 23",
            "20 11 10 24  4",
            "14 21 16 12  6",
            "",
            "14 21 17 24  4",
            "10 16 15  9 19",
            "18  8 23 26 20",
            "22 11 13  6  5",
            "2  0 12  3  7",
        ].into_iter().map(String::from).collect()
    }

    fn sample_board() -> Board {
        Board{
            numbers: vec![
                vec![22, 13, 17, 11,  0],
                vec![ 8,  2, 23,  4, 24],
                vec![21,  9, 14, 16,  7],
                vec![ 6, 10,  3, 18,  5],
                vec![ 1, 12, 20, 15, 19],
            ]
        }
    }

    #[test]
    fn test_no_board_match() {
        assert_eq!(find_match(&sample_board(), 72), None);
    }

    #[test]
    fn test_board_match() {
        assert_eq!(find_match(&sample_board(), 4), Some((1, 3)));
    }

    #[test]
    fn test_read() -> MyResult<()>{
        let (numbers, boards) = read(&small_test_input())?;
        assert_eq!(numbers, [7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1]);
        assert_eq!(boards.len(), 3);
        assert_eq!(boards[0].numbers[0], [22, 13, 17, 11,  0]);
        assert_eq!(boards[1].numbers[2], [19,  8,  7, 25, 23]);
        Ok(())
    }
    #[test]
    fn test_simulate() -> MyResult<()> {
        let (numbers, boards) = read(&small_test_input())?;
        let completed_board_scores = simulate(&numbers, &boards);
        assert_eq!(completed_board_scores[0], 4512);
        assert_eq!(completed_board_scores[completed_board_scores.len() - 1], 1924);
        Ok(())
    }
}