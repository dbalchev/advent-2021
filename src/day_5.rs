
use std::io::BufRead;
use std::fs::File;
use std::io::BufReader;
use ndarray::{Array2, arr2, s};
use std::fmt::Display;
use regex::{Regex, Error as RegexParsingError};
use std::error::Error;
use std::str::FromStr;
use crate::common::{MyResult, make_my_result};

#[derive(PartialEq, Debug, Clone)]
enum Line {
    HorizontalLine {
        y: i32, x1: i32, x2: i32,
    },
    VerticalLine {
        x: i32, y1: i32, y2: i32,
    },
    DiagonalLine {
        x1: i32, y1:i32, slope: i32, delta: i32,
    }
}

#[derive(Debug)]
enum LineParsingError {
    RegexError(RegexParsingError),
    ParseFailed(),
    NoCapture(usize),
    NotParallelOrDiagonal(),
}

impl Display for LineParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        write!(f, "{:?}", self)
     }
}

impl Error for LineParsingError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            LineParsingError::RegexError(e) => Some(e),
            _ => None,
        }
    }
}

impl FromStr for Line {
    type Err = Box<dyn Error>;
    fn from_str(line: &str) -> Result<Line, Box<dyn Error>> { 
        let regex = Regex::new("(\\d+),(\\d+) -> (\\d+),(\\d+)")?;
        let captures = regex.captures(line).ok_or(LineParsingError::ParseFailed())?;
        let x1 = i32::from_str(captures.get(1).ok_or(LineParsingError::NoCapture(1))?.as_str())?;
        let y1 = i32::from_str(captures.get(2).ok_or(LineParsingError::NoCapture(2))?.as_str())?;
        let x2 = i32::from_str(captures.get(3).ok_or(LineParsingError::NoCapture(3))?.as_str())?;
        let y2 = i32::from_str(captures.get(4).ok_or(LineParsingError::NoCapture(4))?.as_str())?;


        let x_delta = x2 - x1;
        let y_delta = y2 - y1;

        if x1 == x2 {
            let (y1, y2) = if y1 <= y2 {(y1, y2)} else {(y2, y1)};
            Ok(Line::VerticalLine{x: x1, y1, y2})
        } else {
            if y1 == y2 {
                let (x1, x2) = if x1 <= x2 {(x1, x2)} else {(x2, x1)};
                Ok(Line::HorizontalLine{y: y1, x1, x2})
            } else {
                if x_delta.abs() != y_delta.abs() {
                    Err(Box::new(LineParsingError::NotParallelOrDiagonal()))
                } else {
                    let (x1, y1, x2, y2) = if x1 < x2 {
                        (x1, y1, x2, y2)
                    } else {
                        (x2, y2, x1, y1)
                    };
                    let slope = if y1 < y2 {1} else {-1};
                    let delta = x2 - x1;
                    Ok(Line::DiagonalLine{x1, y1, slope, delta})
                }
            }
        }

    }
}

impl Line {
    fn max_coord(&self) -> usize {
        match *self {
            Line::HorizontalLine{y, x1, x2} => y.max(x1) as usize,
            Line::VerticalLine{x, y1, y2} => x.max(y2) as usize,
            Line::DiagonalLine{x1, y1, slope, delta} => x1.max(y1.max(y1 + slope * delta)) as usize,
        }
    }
    fn is_diagonal(&self) -> bool {
        match *self{
            Line::DiagonalLine{..} => true,
            _ => false,
        }
    }
}


fn draw_line(bitmap: & mut Array2<i32>, line: &Line) -> () {
    match *line{
        Line::HorizontalLine{y, x1, x2} => {
            let mut view = bitmap.slice_mut(s!(y..=y, x1..=x2));
            view += 1;
        }
        Line::VerticalLine{x, y1, y2} => {
            let mut view = bitmap.slice_mut(s!(y1..=y2, x..=x));
            view += 1;
        }, 
        Line::DiagonalLine{x1, y1, slope, delta} => {
            let mut x = x1;
            let mut y = y1;
            for _ in 0..=delta {
                let mut view = bitmap.slice_mut(s!(y..=y, x..=x));
                view += 1;
                x += 1;
                y += slope;
            }
        },
    }
}

fn solve(lines: &[Line]) -> usize {
    let max_coord = 1 + lines.iter().map(Line::max_coord).max().unwrap();
    let mut bitmap = Array2::zeros([max_coord, max_coord]);
    for line in lines {
        draw_line(&mut bitmap, line);
    }
    bitmap.mapv(|x| if x >= 2 {1} else {0}).sum()
}

pub fn run_me() -> MyResult<()> {
    let inputs = BufReader::new(File::open("inputs/day_5.txt")?)
        .lines()
        .map(make_my_result)
        .collect::<MyResult<Vec<String>>>()?;
    let lines = inputs
        .into_iter()
        .map(|x| Line::from_str(&x))
        .filter(|x| x.is_ok())
        .collect::<MyResult<Vec<Line>>>()?;
    let parallel_lines: Vec<Line> = lines.iter().filter(|x| !x.is_diagonal()).cloned().collect();

    println!("Task 1: {}", solve(&parallel_lines));
    println!("Task 2: {}", solve(&lines));

    Ok(())
}

#[cfg(test)]
mod tests{
    use super::*;

    fn test_input() -> Vec<String> {
        [
            "0,9 -> 5,9",
            "8,0 -> 0,8",
            "9,4 -> 3,4",
            "2,2 -> 2,1",
            "7,0 -> 7,4",
            "6,4 -> 2,0",
            "0,9 -> 2,9",
            "3,4 -> 1,4",
            "0,0 -> 8,8",
            "5,5 -> 8,2",
        ].into_iter().map(String::from).collect()
    }

    #[test]
    fn parse_horizontal_line() -> MyResult<()> {
        assert_eq!(Line::from_str("0,9 -> 5,9")?, Line::HorizontalLine{y:9, x1:0, x2: 5});
        Ok(())
    }

    
    #[test]
    fn parse_vertical_line() -> MyResult<()> {
        assert_eq!(Line::from_str("7,4 -> 7,0")?, Line::VerticalLine{x:7, y1: 0, y2:4});
        Ok(())
    }

    #[test]
    fn parse_diagonal_line() -> MyResult<()> {
        assert_eq!(Line::from_str("9,7 -> 7,9")?, Line::DiagonalLine{x1:7, y1:9, slope: -1, delta:2});
        Ok(())
    }


    #[test]
    fn max_coord() {
        assert_eq!( Line::VerticalLine{x:7, y1: 0, y2:4}.max_coord(), 7);
    }

    #[test]
    fn draw_horizontal_line() {
        let mut bitmap = Array2::ones([3, 3]);
        draw_line(& mut bitmap, &Line::HorizontalLine{y:1, x1:0, x2:1});
        let expected_result = arr2(&[
            [1, 1, 1],
            [2, 2, 1],
            [1, 1, 1],
        ]);
        assert_eq!(bitmap, expected_result);
    }

    #[test]
    fn draw_vertical_line() {
        let mut bitmap = Array2::ones([3, 3]);
        draw_line(& mut bitmap, &Line::VerticalLine{x:0, y1: 1, y2: 2});
        let expected_result = arr2(&[
            [1, 1, 1],
            [2, 1, 1],
            [2, 1, 1],
        ]);
        assert_eq!(bitmap, expected_result);
    }

    #[test]
    fn draw_diagonal_line() {
        let mut bitmap = Array2::ones([3, 3]);
        draw_line(& mut bitmap, &Line::DiagonalLine{x1:1, y1: 1, slope:-1, delta:1});
        let expected_result = arr2(&[
            [1, 1, 2],
            [1, 2, 1],
            [1, 1, 1],
        ]);
        assert_eq!(bitmap, expected_result);
    }
    
    #[test]
    fn test_solve() -> MyResult<()> {
        let lines = test_input()
            .into_iter()
            .map(|x| Line::from_str(&x))
            .collect::<MyResult<Vec<Line>>>()?;
        let parallel_lines: Vec<Line> = lines.iter().filter(|x| !x.is_diagonal()).cloned().collect();
        assert_eq!(solve(&parallel_lines), 5);
        Ok(())
    }

    #[test]
    fn test_solve_2() -> MyResult<()> {
        let lines = test_input()
            .into_iter()
            .map(|x| Line::from_str(&x))
            .collect::<MyResult<Vec<Line>>>()?;
        assert_eq!(solve(&lines), 12);
        Ok(())
    }
}
