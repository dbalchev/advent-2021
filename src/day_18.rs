use crate::day_18::SnailFish::{Pair, Regular};
use crate::MyResult;
use std::io::BufRead;

#[derive(Debug, PartialEq, Eq)]
enum SnailFish {
    Regular(i32),
    Pair(i32, Box<SnailFish>, Box<SnailFish>),
}

impl SnailFish {
    fn depth(&self) -> i32 {
        match self {
            Regular(..) => 0,
            Pair(d, ..) => *d,
        }
    }
    fn pair(l: SnailFish, r: SnailFish) -> SnailFish {
        Pair(l.depth().max(r.depth()) + 1, Box::new(l), Box::new(r))
    }
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

// fn force_explode_deepest(x: SnailFish) -> (SnailFish, Option<i32>, Option<i32>) {
//     match x {
//         Regular(..) => panic!("explode should be called on pair"),
//         Pair(_, l, r) => {
//             if l.depth() == 0 && r.depth
//         }
//     }
// }

// fn explode(x: SnailFish) -> SnailFish {
//     if x.depth() < 4 {
//         return x;
//     }

// }

pub fn run_me(reader: impl BufRead) -> MyResult<()> {
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
            (Pair(1, Box::new(Regular(3)), Box::new(Regular(4))), "")
        );
        Ok(())
    }
    #[test]
    fn test_2_pairs() -> MyResult<()> {
        assert_eq!(
            read_snail_fish_full("[[3,4],5]")?,
            (
                Pair(
                    2,
                    Box::new(Pair(1, Box::new(Regular(3)), Box::new(Regular(4)))),
                    Box::new(Regular(5))
                ),
                ""
            )
        );
        Ok(())
    }

    // #[test]
    // fn test_explode_left() -> MyResult<()> {
    //     assert_eq!(
    //         explode(read_snail_fish("[[[[[9,8],1],2],3],4]")?),
    //         read_snail_fish("[[[[0,9],2],3],4]")?,
    //     );
    //     Ok(())
    // }

    // #[test]
    // fn test_explode_right() -> MyResult<()> {
    //     assert_eq!(
    //         explode(read_snail_fish("[7,[6,[5,[4,[3,2]]]]]")?),
    //         read_snail_fish("[7,[6,[5,[7,0]]]]")?,
    //     );
    //     Ok(())
    // }

    // #[test]
    // fn test_explode_mid() -> MyResult<()> {
    //     assert_eq!(
    //         explode(read_snail_fish("[[6,[5,[4,[3,2]]]],1]")?),
    //         read_snail_fish("[[6,[5,[7,0]]],3]")?,
    //     );
    //     Ok(())
    // }
}
