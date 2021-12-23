use itertools::Itertools;

use crate::day_18::SnailFish::{Pair, Regular};
use crate::MyResult;
use std::io::BufRead;
use std::mem::replace;

#[derive(Debug, PartialEq, Eq, Clone)]
enum SnailFish {
    Regular(i32),
    Pair([Box<SnailFish>; 2]),
}

impl SnailFish {
    fn pair(l: SnailFish, r: SnailFish) -> SnailFish {
        Pair([Box::new(l), Box::new(r)])
    }

    fn find_explode_node(&mut self, path: &mut Vec<i32>) -> Option<([i32; 2], Vec<i32>)> {
        match self {
            Regular(..) => return None,
            Pair(children) => {
                if path.len() == 4 {
                    let old_self = replace(self, Regular(0));
                    if let Pair(children) = old_self {
                        let [l, r] = children;
                        let lr = [*l, *r];
                        if let [Regular(l), Regular(r)] = lr {
                            return Some(([l, r], path.clone()));
                        } else {
                            unreachable!("not expected {:?}", lr);
                        }
                    } else {
                        unreachable!("not expected {:?}", old_self);
                    }
                }
                path.push(0);
                let x = children[0].find_explode_node(path);
                if matches!(x, Some(..)) {
                    return x;
                }
                *path.last_mut().unwrap() += 1;
                let x = children[1].find_explode_node(path);
                if matches!(x, Some(..)) {
                    return x;
                }
                path.pop();
                return None;
            }
        }
    }

    fn walk_path_and_add(&mut self, path: &[i32], num: i32) {
        match self {
            Regular(x) => *x += num,
            Pair(children) => children[path[0] as usize].walk_path_and_add(&path[1..], num),
        }
    }

    fn explode(&mut self) -> bool {
        let mut path = Vec::new();
        if let Some(([l, r], path)) = self.find_explode_node(&mut path) {
            if let Some(l_path) = prepare_path(path.clone(), 0) {
                self.walk_path_and_add(&l_path, l);
            }
            if let Some(r_path) = prepare_path(path, 1) {
                self.walk_path_and_add(&r_path, r);
            }
            true
        } else {
            false
        }
    }

    fn split(&mut self) -> bool {
        match self {
            Regular(num) if *num >= 10 => {
                let num = *num;
                drop(replace(
                    self,
                    SnailFish::pair(Regular(num / 2), Regular((num + 1) / 2)),
                ));
                true
            }
            Regular(..) => false,
            Pair(children) => {
                for child in children {
                    if child.split() {
                        return true;
                    }
                }
                false
            }
        }
    }

    fn reduce(&mut self) {
        loop {
            if self.explode() {
                continue;
            }
            if self.split() {
                continue;
            }
            break;
        }
    }

    fn compute_magnitude(&self) -> i32 {
        match self {
            Regular(num) => *num,
            Pair([l, r]) => 3 * l.compute_magnitude() + 2 * r.compute_magnitude(),
        }
    }

    fn add(self, rh: SnailFish) -> SnailFish {
        let mut sum = SnailFish::pair(self, rh);
        sum.reduce();
        return sum;
    }
}

fn prepare_path(mut path: Vec<i32>, current_dir: i32) -> Option<Vec<i32>> {
    let other_dir = 1 - current_dir;
    while let Some(dir) = path.pop() {
        if dir == other_dir {
            path.push(current_dir);
            break;
        }
    }
    if path.len() > 0 {
        path.extend_from_slice(&[other_dir; 4]);
        return Some(path);
    }
    return None;
}

fn read_snail_fish_full(input: &str) -> MyResult<(SnailFish, &str)> {
    match input.chars().next().ok_or(format!("unexpected end"))? {
        c @ ('0'..='9') => Ok((Regular(c as i32 - '0' as i32), &input[1..])),
        '[' => {
            let (first_part, rest) = read_snail_fish_full(&input[1..])?;
            let mid_char = rest.chars().next().ok_or(format!("unexpected end mid"))?;
            if mid_char != ',' {
                Err(format!("unexpected mid char {}", mid_char))?
            }
            let (second_part, rest) = read_snail_fish_full(&rest[1..])?;
            let end_char = rest.chars().next().ok_or(format!("unexpected end last"))?;
            if end_char != ']' {
                Err(format!("unexpected end char {}", end_char))?
            }
            Ok((SnailFish::pair(first_part, second_part), &rest[1..]))
        }
        c @ _ => Err(format!("unexpected start char {}", c))?,
    }
}

fn read_snail_fish(input: &str) -> MyResult<SnailFish> {
    let (sf, rest) = read_snail_fish_full(input)?;
    if rest != "" {
        Err(format!("unexpected rest {}", rest))?
    }
    Ok(sf)
}

pub fn run_me(reader: impl BufRead) -> MyResult<()> {
    let numbers = reader
        .lines()
        .map(|x| Ok(read_snail_fish(&x?)?))
        .collect::<MyResult<Vec<_>>>()?;

    println!(
        "Task 1: {}",
        numbers
            .iter()
            .cloned()
            .reduce(|l, r| l.add(r))
            .unwrap()
            .compute_magnitude()
    );
    println!(
        "Task 2: {:?}",
        numbers
            .iter()
            .combinations(2)
            .flat_map(|v| [(v[0].clone(), v[1].clone()), (v[1].clone(), v[0].clone())])
            .map(|(l, r)| l.add(r).compute_magnitude())
            .max()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regular() -> MyResult<()> {
        assert_eq!(read_snail_fish_full("5")?, (Regular(5), ""));
        Ok(())
    }

    #[test]
    fn test_pair() -> MyResult<()> {
        assert_eq!(
            read_snail_fish_full("[3,4]")?,
            (Pair([Box::new(Regular(3)), Box::new(Regular(4))]), "")
        );
        Ok(())
    }
    #[test]
    fn test_2_pairs() -> MyResult<()> {
        assert_eq!(
            read_snail_fish_full("[[3,4],5]")?,
            (
                Pair([
                    Box::new(Pair([Box::new(Regular(3)), Box::new(Regular(4))])),
                    Box::new(Regular(5))
                ]),
                ""
            )
        );
        Ok(())
    }

    #[test]
    fn test_explode_left() -> MyResult<()> {
        let mut ex = read_snail_fish("[[[[[9,8],1],2],3],4]")?;
        ex.explode();
        assert_eq!(ex, read_snail_fish("[[[[0,9],2],3],4]")?,);
        Ok(())
    }

    #[test]
    fn test_explode_right() -> MyResult<()> {
        let mut ex = read_snail_fish("[7,[6,[5,[4,[3,2]]]]]")?;
        ex.explode();
        assert_eq!(ex, read_snail_fish("[7,[6,[5,[7,0]]]]")?,);
        Ok(())
    }

    #[test]
    fn test_explode_mid() -> MyResult<()> {
        let mut ex = read_snail_fish("[[6,[5,[4,[3,2]]]],1]")?;
        ex.explode();
        assert_eq!(ex, read_snail_fish("[[6,[5,[7,0]]],3]")?,);
        Ok(())
    }
    #[test]
    fn test_add() -> MyResult<()> {
        assert_eq!(
            read_snail_fish("[[[[4,3],4],4],[7,[[8,4],9]]]")?.add(read_snail_fish("[1,1]")?),
            read_snail_fish("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")?,
        );
        Ok(())
    }
}
