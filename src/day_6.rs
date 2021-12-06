use crate::MyResult;
use std::error::Error;
use std::fmt::Display;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

const MAX_COUNTER: usize = 9;
type CounterFrequency = Vec<usize>;

#[derive(Debug)]
struct FishOverflow();

impl Display for FishOverflow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

impl Error for FishOverflow {}

fn parse_input(input: &str) -> MyResult<CounterFrequency> {
    let mut result = vec![0; MAX_COUNTER];
    for counter_string in input.split(',') {
        let counter = usize::from_str(counter_string)?;
        result[counter] += 1;
    }
    Ok(result)
}

fn compute_next_tick(mut counter_frequency: CounterFrequency) -> MyResult<CounterFrequency> {
    counter_frequency.rotate_left(1);
    counter_frequency[6] = counter_frequency[6]
        .checked_add(counter_frequency[8])
        .ok_or(FishOverflow())?;
    Ok(counter_frequency)
}

fn solve(input: &str, num_steps: usize) -> MyResult<usize> {
    let mut counter_frequency = parse_input(input)?;
    for _ in 0..num_steps {
        counter_frequency = compute_next_tick(counter_frequency)?;
    }
    Ok(counter_frequency
        .into_iter()
        .try_fold(0usize, |acc, x| acc.checked_add(x).ok_or(FishOverflow()))?)
}

pub fn run_me() -> MyResult<()> {
    let input = BufReader::new(File::open("inputs/day_6.txt")?)
        .lines()
        .next()
        .unwrap()?;

    println!("Task 1: {}", solve(&input, 80)?);
    println!("Task 2: {}", solve(&input, 256)?);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_input() -> MyResult<()> {
        assert_eq!(parse_input("3,4,3,1,2")?, vec![0, 1, 1, 2, 1, 0, 0, 0, 0],);
        Ok(())
    }
    #[test]
    fn test_compute_next_tick() -> MyResult<()> {
        assert_eq!(
            compute_next_tick(vec![3, 0, 2, 0, 1, 0, 0, 5, 0])?,
            vec![0, 2, 0, 1, 0, 0, 8, 0, 3],
        );
        Ok(())
    }

    #[test]
    fn test_solve_1() -> MyResult<()> {
        assert_eq!(solve("3,4,3,1,2", 80)?, 5934,);
        Ok(())
    }

    #[test]
    fn test_solve_2() -> MyResult<()> {
        assert_eq!(solve("3,4,3,1,2", 256)?, 26984457539,);
        Ok(())
    }
}
