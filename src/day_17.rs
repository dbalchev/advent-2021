use crate::MyResult;
use itertools::iterate;
use itertools::Itertools;
use regex::Regex;
use std::io::BufRead;
use std::ops::RangeInclusive;
use std::str::FromStr;

fn simulate(
    ivx: i32,
    ivy: i32,
    x_target: &RangeInclusive<i32>,
    y_target: &RangeInclusive<i32>,
) -> Option<i32> {
    let step = |&(x, y, vx, vy, my): &(i32, i32, i32, i32, i32)| {
        return (x + vx, y + vy, (vx - 1).max(0), vy - 1, y.max(my));
    };
    iterate((0, 0, ivx, ivy, 0), step)
        // .inspect(|t| println!("{:?} {:?} {:?}", t, x_target.end(), y_target.end()))
        .take_while(|&(x, y, _, _, _)| x <= *x_target.end() && y >= *y_target.start())
        .filter(|(x, y, _, _, my)| x_target.contains(&x) && y_target.contains(&y))
        // .inspect(|t| println!("{:?}", t))
        .map(|t| t.4)
        .next()
}

pub fn run_me(reader: impl BufRead) -> MyResult<()> {
    let input_line = reader.lines().next().ok_or(format!("no input line"))??;
    let regex = Regex::new("target area: x=(-?\\d+)..(-?\\d+), y=(-?\\d+)..(-?\\d+)")?;
    let captures = regex
        .captures(&input_line)
        .ok_or(format!("no match {}", input_line))?;
    let read = |i| i32::from_str(&captures[i]);
    let x_target = read(1)?..=read(2)?;
    let y_target = read(3)?..=read(4)?;
    println!("{:?} {:?}", x_target, y_target);

    let my = (0..1000)
        .cartesian_product(0..1000)
        .filter_map(|(ivx, ivy)| simulate(ivx, ivy, &x_target, &y_target))
        .max();

    println!("Task 1: {:?}", my);

    let nys = (-1000..1000)
        .cartesian_product(-1000..1000)
        .filter_map(|(ivx, ivy)| simulate(ivx, ivy, &x_target, &y_target))
        .count();

    println!("Task 2: {:?}", nys);

    // println!("{:?}", simulate(6, 9, &x_target, &y_target));
    Ok(())
}
