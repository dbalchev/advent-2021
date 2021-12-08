use crate::MyResult;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env::args;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

type CharSet = HashSet<char>;

#[derive(Debug, PartialEq)]
struct NoteLine {
    signal_patterns: [CharSet; 10],
    digit_outputs: [CharSet; 4],
}

fn char_set(chars: &str) -> CharSet {
    let mut chars_vec: Vec<char> = chars.chars().collect();
    chars_vec.sort();
    return chars_vec.into_iter().collect();
}

fn parse_note_line(line: &str) -> NoteLine {
    let (patterns, digits) = line.split_once(" | ").unwrap();
    return NoteLine {
        signal_patterns: TryFrom::try_from(
            patterns
                .split_whitespace()
                .map(char_set)
                .collect::<Vec<CharSet>>(),
        )
        .unwrap(),
        digit_outputs: TryFrom::try_from(
            digits
                .split_whitespace()
                .map(char_set)
                .collect::<Vec<CharSet>>(),
        )
        .unwrap(),
    };
}

fn find_with_filter<'a, F>(
    initial_set: Vec<&'a CharSet>,
    f: F,
    expected_count: usize,
) -> Vec<&'a CharSet>
where
    F: FnMut(&&'a CharSet) -> bool,
{
    let possibilities: Vec<&CharSet> = initial_set.into_iter().filter(f).collect();
    assert_eq!(possibilities.len(), expected_count);
    return possibilities;
}

fn sorted_vec(x: &CharSet) -> Vec<char> {
    let mut v: Vec<char> = x.iter().cloned().collect();
    v.sort();
    v
}

fn solve_line(x: &NoteLine) -> i32 {
    let find_with_len = |len, expected_count| {
        let possibilities: Vec<&CharSet> = x
            .signal_patterns
            .iter()
            .filter(|x| x.len() == len)
            .collect();
        assert_eq!(possibilities.len(), expected_count);
        return possibilities;
    };
    let r1 = find_with_len(2, 1)[0];
    let r7 = find_with_len(3, 1)[0];
    let r4 = find_with_len(4, 1)[0];
    let r8 = find_with_len(7, 1)[0];

    let len_6 = find_with_len(6, 3);
    let len_5 = find_with_len(5, 3);
    let r_b_or_d = r4 - r1;
    assert_eq!(r_b_or_d.len(), 2);

    let r0 = find_with_filter(len_6.clone(), |x| !x.is_superset(&r_b_or_d), 1)[0];
    let rb: CharSet = r0.intersection(&r_b_or_d).cloned().collect();
    assert_eq!(rb.len(), 1);
    let r5 = find_with_filter(len_5.clone(), |x| x.is_superset(&rb), 1)[0];
    let r_2_or_3 = find_with_filter(len_5, |x| x != &r5, 2);
    let r_e_or_f: CharSet = r_2_or_3[0]
        .symmetric_difference(&r_2_or_3[1])
        .cloned()
        .collect();
    assert_eq!(r_e_or_f.len(), 2);
    let r_e = &r_e_or_f - r1;
    assert_eq!(r_e.len(), 1);
    let r2 = find_with_filter(r_2_or_3.clone(), |x| x.intersection(&r_e).count() > 0, 1)[0];
    let r3 = find_with_filter(r_2_or_3, |x| x.intersection(&r_e).count() == 0, 1)[0];
    let r_6_or_9 = find_with_filter(len_6, |x| x != &r0, 2);
    let r6 = find_with_filter(r_6_or_9.clone(), |x| x.intersection(&r_e).count() > 0, 1)[0];
    let r9 = find_with_filter(r_6_or_9.clone(), |x| x.intersection(&r_e).count() == 0, 1)[0];

    let map = HashMap::<Vec<char>, char>::from([
        (sorted_vec(r0), '0'),
        (sorted_vec(r1), '1'),
        (sorted_vec(r2), '2'),
        (sorted_vec(r3), '3'),
        (sorted_vec(r4), '4'),
        (sorted_vec(r5), '5'),
        (sorted_vec(r6), '6'),
        (sorted_vec(r7), '7'),
        (sorted_vec(r8), '8'),
        (sorted_vec(r9), '9'),
    ]);
    let decoded: String = x
        .digit_outputs
        .iter()
        .map(|x| *map.get(&sorted_vec(&x)).unwrap())
        .collect();
    i32::from_str(&decoded).unwrap()
}

pub fn run_me() -> MyResult<()> {
    let notes_lines = BufReader::new(File::open(args().nth(1).unwrap())?)
        .lines()
        .map(|x| Ok(parse_note_line(&x?)))
        .collect::<MyResult<Vec<NoteLine>>>()?;

    let task_1_solution = notes_lines
        .iter()
        .flat_map(|x| x.digit_outputs.iter())
        .filter(|x| matches!(x.len(), 2 | 3 | 4 | 7))
        .count();
    println!("Task 1: {}", task_1_solution);

    let task_2_solution: i32 = notes_lines.iter().map(solve_line).sum();
    println!("Task 2: {}", task_2_solution);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn note_line() -> NoteLine {
        NoteLine {
            signal_patterns: [
                char_set("acedgfb"),
                char_set("cdfbe"),
                char_set("gcdfa"),
                char_set("fbcad"),
                char_set("dab"),
                char_set("cefabd"),
                char_set("cdfgeb"),
                char_set("eafb"),
                char_set("cagedb"),
                char_set("ab"),
            ],
            digit_outputs: [
                char_set("cdfeb"),
                char_set("fcadb"),
                char_set("cdfeb"),
                char_set("cdbaf"),
            ],
        }
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            parse_note_line("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"),
            note_line(),
        );
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve_line(&note_line()), 5353,)
    }
}
