use std::io::BufRead;
use std::fs::File;
use std::io::BufReader;
use std::io;



fn find_counts(input: &Vec<String>) -> Vec<(usize, usize)> {
    let item_len = input[0].len();
    let mut one_counts = vec![0usize; item_len];
    for report_line in input {
        for (current_count, current_char) in one_counts.iter_mut().zip(report_line.chars()) {
            if current_char == '1' {
                *current_count += 1;
            }
        }
    }
    return one_counts.into_iter().map(|x| (input.len() - x, x)).collect();
}
fn find_most_common(input: &Vec<String>) -> String {
    return find_counts(input).into_iter().map(|(c0, c1)| if c0 <= c1 {'1'} else {'0'}).collect();
}

fn solve_1(most_common: String) -> i64 {
    let gamma = i64::from_str_radix(&most_common, 2).unwrap();
    let epsilon = (1 << most_common.len()) - 1 - gamma;
    // println!("{} {}", gamma, epsilon);
    return gamma * epsilon;
}

fn o2_desired_char(zero_counts: usize, one_counts: usize) -> char {
    if one_counts >= zero_counts {'1'} else {'0'}
}

fn co2_desired_char(zero_counts: usize, one_counts: usize) -> char {
    if one_counts < zero_counts {'1'} else {'0'}
}

fn compute_rating<CDC>(input: &Vec<String>, compute_desired_char: CDC) -> String
where CDC: Fn(usize, usize) -> char {
    let mut current_strings:Vec<Vec<char>> = input.into_iter().map(|x| x.chars().collect()).collect();
    for bit_no in 0..input[0].len() {
        let one_counts = current_strings.iter().filter(|x| x[bit_no] == '1').count();
        let zero_counts = current_strings.len() - one_counts;
        let desired_char = compute_desired_char(zero_counts, one_counts);
        current_strings = current_strings.into_iter().filter(|x| x[bit_no] == desired_char).collect();
        if current_strings.len() == 1 {
            break;
        }
    }
    return String::from_iter(current_strings[0].iter());
}

fn solve_2(input: &Vec<String>) -> i64 {
    let o2_rating = i64::from_str_radix(&compute_rating(input, o2_desired_char), 2).unwrap();
    let co2_rating = i64::from_str_radix(&compute_rating(input, co2_desired_char), 2).unwrap();

    return o2_rating * co2_rating;
}

pub fn run_me() -> io::Result<()> {
    let inputs = BufReader::new(File::open("inputs/day_3.txt")?).lines().map(Result::unwrap).collect();
    let most_common = find_most_common(&inputs);
    println!("Solution 1: {}", solve_1(most_common));
    println!("Solution 2: {}", solve_2(&inputs));
    Ok(())
}

#[cfg(test)]
mod tests{
    use super::*;
    fn small_input() -> Vec<String> {
      vec![
            String::from("00100"),
            String::from("11110"),
            String::from("10110"),
            String::from("10111"),
            String::from("10101"),
            String::from("01111"),
            String::from("00111"),
            String::from("11100"),
            String::from("10000"),
            String::from("11001"),
            String::from("00010"),
            String::from("01010"),
        ]
    }
    #[test]
    fn test_find_most_common(){
        assert_eq!(find_most_common(&small_input()), String::from("10110"));
    }

    #[test]
    fn test_solve_1() {
        assert_eq!(solve_1(String::from("10110")), 198);
    }

    #[test]
    fn test_compute_rating(){
        assert_eq!(compute_rating(&small_input(), o2_desired_char), String::from("10111"));
        assert_eq!(compute_rating(&small_input(), co2_desired_char), String::from("01010"));

    }

    #[test]
    fn test_solve_2(){
        assert_eq!(solve_2(&small_input()), 230);
    }
}