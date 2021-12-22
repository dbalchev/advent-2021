use core::num;
use std::{
    collections::HashMap,
    io::BufRead,
    ops::{Range, RangeInclusive},
};

use crate::common::MyResult;
use itertools::Itertools;
use ndarray::{s, Array3};
use regex::Regex;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Instruction<R: Clone> {
    new_value: bool,
    xs: R,
    ys: R,
    zs: R,
}

struct Remapper {
    m: HashMap<i32, usize>,
    numbers: Vec<i32>,
}

impl Remapper {
    fn new(ranges: impl Iterator<Item = RangeInclusive<i32>>) -> Self {
        let mut numbers = ranges
            .flat_map(|a| [*a.start(), *a.end() + 1])
            .collect_vec();
        numbers.sort();
        numbers.dedup();
        Remapper {
            m: numbers
                .iter()
                .cloned()
                .enumerate()
                .map(|(i, v)| (v, i))
                .collect(),
            numbers,
        }
    }

    fn map(&self, r: RangeInclusive<i32>) -> Range<usize> {
        self.m[&r.start()]..self.m[&(r.end() + 1)]
    }

    fn region_size(&self, r: usize) -> i32 {
        self.numbers[r + 1] - self.numbers[r]
    }
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

    let x_remapper = Remapper::new(instructions.iter().map(|i| i.xs.clone()));
    let y_remapper = Remapper::new(instructions.iter().map(|i| i.ys.clone()));
    let z_remapper = Remapper::new(instructions.iter().map(|i| i.zs.clone()));

    let remapped_instructions = instructions
        .iter()
        .cloned()
        .map(
            |Instruction {
                 new_value,
                 xs,
                 ys,
                 zs,
             }| Instruction {
                new_value,
                xs: x_remapper.map(xs),
                ys: y_remapper.map(ys),
                zs: z_remapper.map(zs),
            },
        )
        .collect_vec();

    let mut remapped_reactor = Array3::from_elem(
        [
            x_remapper.numbers.len() - 1,
            y_remapper.numbers.len() - 1,
            z_remapper.numbers.len() - 1,
        ],
        false,
    );

    for Instruction {
        new_value,
        xs,
        ys,
        zs,
    } in &remapped_instructions
    {
        remapped_reactor
            .slice_mut(s![xs.clone(), ys.clone(), zs.clone()])
            .fill(*new_value);
    }
    println!(
        "Task 2: {}",
        remapped_reactor
            .indexed_iter()
            .map(|((i, j, k), v)| (*v as u128)
                * (x_remapper.region_size(i) as u128)
                * (y_remapper.region_size(j) as u128)
                * (z_remapper.region_size(k) as u128))
            .sum::<u128>()
    );

    Ok(())
}
