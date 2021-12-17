
use itertools::Itertools;
use itertools::iterate;
use std::ops::RangeInclusive;
use std::str::FromStr;
use regex::Regex;
use crate::MyResult;
use std::io::BufRead;

fn simulate(ivx: i32, ivy: i32, x_target: &RangeInclusive<i32>, y_target: &RangeInclusive<i32>) -> Option<i32> {
    let step = |&(x, y, vx, vy, my): &(i32, i32, i32, i32, i32)| {
        return (x + vx, y + vy, vx - 1, vy - 1, y.max(my));
    };
    if let Some((x, y, _, _, my)) = iterate((0, 0, ivx, ivy, 0), step).take_while(
        |&(x, y, _, _, _)| {
            // println!("{} {} {:?}", x, y, x_target);
            x < *x_target.start()
        }
    ).next() {
        if x_target.contains(&x) && y_target.contains(&y) {
            return Some(my)
        }
    }

    return None

}

pub fn run_me(reader: impl BufRead) -> MyResult<()> {
    let input_line = reader.lines().next().ok_or(format!("no input line"))??;
    let regex = Regex::new("target area: x=(-?\\d+)..(-?\\d+), y=(-?\\d+)..(-?\\d+)")?;
    let captures = regex.captures(&input_line).ok_or(format!("no match {}", input_line))?;
    let read = |i| {i32::from_str(&captures[i])};
    let x_target = read(1)?..=read(2)?;
    let y_target = read(3)?..=read(4)?;
    println!("{:?} {:?}", x_target, y_target);

    let my = (0..100).cartesian_product(0..100).filter_map(|(ivx, ivy)| simulate(ivx, ivy, &x_target, &y_target)).max();

    println!("Task 1: {:?}", my);

    Ok(())
}