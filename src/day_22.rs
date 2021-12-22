use std::{io::BufRead, ops::RangeInclusive};

use crate::common::MyResult;
use ndarray::{s, Array3};
use regex::Regex;
use std::str::FromStr;

#[derive(Debug)]
struct Instruction {
    new_value: bool,
    xs: RangeInclusive<i32>,
    ys: RangeInclusive<i32>,
    zs: RangeInclusive<i32>,
}

pub fn run_me(reader: impl BufRead) -> MyResult<()> {
    let parse_regex =
        Regex::new("(on|off) x=(-?\\d+)..(-?\\d+),y=(-?\\d+)..(-?\\d+),z=(-?\\d+)..(-?\\d+)")?;

    let instructions = reader
        .lines()
        .map(|line| {
            let line = line?;
            let captures = parse_regex
                .captures(&line)
                .ok_or(format!("not matching '{}'", line))?
                .iter()
                .map(|c| Ok(c.ok_or("no match")?.as_str()))
                .collect::<MyResult<Vec<_>>>()?;
            let [_, nv, xb, xe, yb, ye, zb, ze]: [_; 8] = captures
                .try_into()
                .map_err(|v: Vec<_>| format!("has len {}", v.len()))?;

            Ok(Instruction {
                new_value: nv == "on",
                xs: i32::from_str(xb)?..=i32::from_str(xe)?,
                ys: i32::from_str(yb)?..=i32::from_str(ye)?,
                zs: i32::from_str(zb)?..=i32::from_str(ze)?,
            })
        })
        .collect::<MyResult<Vec<_>>>()?;
    // println!("{:?}", instructions);

    let mut reactor = Array3::from_elem([101, 101, 101], false);

    for Instruction {
        new_value,
        xs,
        ys,
        zs,
    } in &instructions
    {
        // println!("a {:?} {:?} {:?}", xs, ys, zs);

        let xs = (xs.start().clamp(&-50, &51) + 50)..((xs.end() + 1).clamp(-50, 51) + 50);
        let ys = (ys.start().clamp(&-50, &51) + 50)..((ys.end() + 1).clamp(-50, 51) + 50);
        let zs = (zs.start().clamp(&-50, &51) + 50)..((zs.end() + 1).clamp(-50, 51) + 50);

        // println!("b {:?} {:?} {:?}", xs, ys, zs);
        reactor.slice_mut(s![xs, ys, zs]).fill(*new_value);
    }

    println!("Task 1: {}", reactor.mapv(|x| x as i32).sum());

    Ok(())
}
