use std::collections::HashMap;
use std::io::BufRead;
use std::str::FromStr;

use crate::common::MyResult;
use itertools::Itertools;
use ndarray::Array2;
use ndarray::ArrayD;
use ndarray::Axis;

use self::Arg::*;
use self::Instruction::*;
type RegisterName = char;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Arg {
    Register(RegisterName),
    Number(i32),
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Inp(RegisterName),
    Add(RegisterName, Arg),
    Mul(RegisterName, Arg),
    Div(RegisterName, Arg),
    Mod(RegisterName, Arg),
    Eql(RegisterName, Arg),
}

fn is_matching_arg(lh: &Arg, rh: &Arg) -> bool {
    match (lh, rh) {
        (Register(l), Register(r)) => l == r,
        (Number(..), Number(..)) => true,
        _ => false,
    }
}

fn is_matching(lh: &Instruction, rh: &Instruction) -> bool {
    match (lh, rh) {
        (Inp(l), Inp(r)) => l == r,
        (Add(la, lb), Add(ra, rb)) => (la == ra) && is_matching_arg(lb, rb),
        (Mul(la, lb), Mul(ra, rb)) => (la == ra) && is_matching_arg(lb, rb),
        (Div(la, lb), Div(ra, rb)) => (la == ra) && is_matching_arg(lb, rb),
        (Mod(la, lb), Mod(ra, rb)) => (la == ra) && is_matching_arg(lb, rb),
        (Eql(la, lb), Eql(ra, rb)) => (la == ra) && is_matching_arg(lb, rb),
        _ => false,
    }
}

fn parse(line: &str) -> Instruction {
    let line_parts = line.split_whitespace().collect_vec();

    let line_parts = match line_parts.try_into() as Result<[&str; 2], Vec<&str>> {
        Ok([op, a]) => {
            assert_eq!(op, "inp");
            assert_eq!(a.len(), 1);
            return Inp(a.chars().next().unwrap());
        }
        Err(line_parts) => line_parts,
    };
    let [op, a, b] = (line_parts.try_into() as Result<[&str; 3], Vec<&str>>)
        .map_err(|x| format!("unexpected len {} ", x.len()))
        .unwrap();
    assert_eq!(a.len(), 1);
    let a = a.chars().next().unwrap();
    let b = match i32::from_str(b) {
        Ok(num) => Number(num),
        Err(..) => {
            assert_eq!(b.len(), 1);
            Register(b.chars().next().unwrap())
        }
    };
    match op {
        "add" => Add(a, b),
        "mul" => Mul(a, b),
        "div" => Div(a, b),
        "mod" => Mod(a, b),
        "eql" => Eql(a, b),
        _ => unreachable!("unexpected op {}", op),
    }
}

fn infix_expressioin(vm: &mut HashMap<char, String>, a: char, op: &str, b: Arg) {
    let b = match b {
        Register(b) => vm[&b].clone(),
        Number(n) => n.to_string(),
    };
    let a_val = vm.get_mut(&a).unwrap();
    let a_needs_parens = a_val.contains(&['+', '*', '/', '%', '='][..]);
    let b_needs_parens = b.contains(&['+', '*', '/', '%', '='][..]);
    if a_needs_parens {
        a_val.insert(0, '(');
        a_val.push(')');
    }

    a_val.push_str(&format!(" {} ", op));
    if b_needs_parens {
        a_val.push('(');
    }
    a_val.push_str(&b);
    if b_needs_parens {
        a_val.push(')');
    }
}

const C1: [i32; 14] = [1, 1, 1, 1, 26, 1, 26, 1, 1, 26, 26, 26, 26, 26];
const C2: [i32; 14] = [11, 14, 15, 13, -12, 10, -15, 13, 10, -13, -13, -14, -2, -9];
const C3: [i32; 14] = [14, 6, 6, 13, 8, 8, 7, 10, 8, 12, 10, 8, 8, 7];

fn deduced_op(z: i32, w: i32, step_no: usize) -> i32 {
    let c1 = C1[step_no];
    let c2 = C2[step_no];
    let c3 = C3[step_no];
    let x = z % 26 + c2;

    let xx = (x != w) as i32;

    let z = (z / c1) * (25 * xx + 1) + (w + c3) * xx;

    return z;
}

pub fn run_me(reader: impl BufRead) -> MyResult<()> {
    let instructions = reader
        .lines()
        .map_ok(|x| parse(&x))
        .collect::<Result<Vec<_>, _>>()?;
    // println!("{:?}", instructions);

    // analyze_instructions(instructions);
    let max_z = 20_000_000;
    let min_z: i32 = 0;
    let mut memo = Array2::from_elem((15, (max_z - min_z) as usize), Vec::new());

    memo[[0, (-min_z) as usize]].push((-1, 0));

    for step in 0..14 {
        for z_value in min_z..max_z {
            if memo[[step, (z_value - min_z) as usize]].len() == 0 {
                continue;
            }
            for w in 1..10 {
                let new_z = deduced_op(z_value as i32, w, step);
                // println!("z = {}", new_z);
                if (0..max_z as i32).contains(&new_z) {
                    memo[[step + 1, (new_z - min_z) as usize]].push((w, z_value))
                } else {
                    // println!("z = {}", new_z);
                }
            }
        }
    }

    println!(
        "{:?}",
        memo.mapv(|x| (x.len() > 0) as i32)
            .axis_iter(Axis(0))
            .map(|x| x.sum())
            .collect_vec()
    );
    println!("{:?}", memo[[14, (-min_z) as usize]]);

    let mut reverse_memo = Array2::from_elem((15, (max_z) as usize), Vec::new());

    reverse_memo[[14, 0]].push((-1, -1));

    for step in (1..=14).rev() {
        for z_value in 0..max_z {
            if reverse_memo[[step, z_value as usize]].len() > 0 {
                for &(prev_char, prev_z) in &memo[[step, z_value as usize]] {
                    reverse_memo[[step - 1, prev_z as usize]].push((prev_char, z_value));
                }
            }
        }
    }
    println!(
        "{:?}",
        reverse_memo
            .mapv(|x| (x.len() > 0) as i32)
            .axis_iter(Axis(0))
            .map(|x| x.sum())
            .collect_vec()
    );
    let mut beam = reverse_memo
        .index_axis(Axis(0), 0)
        .iter()
        .filter(|x| x.len() > 0)
        .nth(0)
        .unwrap()
        .clone();

    println!("{:?}", beam);
    let mut serial_code = Vec::new();
    for step in 0..14 {
        let max_digit = beam.iter().map(|&(digit, _)| digit).max().unwrap();
        beam.retain(|&(digit, _)| digit == max_digit);
        serial_code.push(max_digit);
        beam = beam
            .into_iter()
            .flat_map(|(_, next_z)| reverse_memo[[step as usize + 1, next_z as usize]].clone())
            .collect_vec();
    }
    println!("{}", serial_code.len());
    println!("{:?}", serial_code);
    println!("{:?}", serial_code.into_iter().join(""));
    Ok(())
}

fn analyze_instructions(instructions: Vec<Instruction>) {
    for delta in (18..252).step_by(18) {
        let m = instructions[0..18]
            .iter()
            .zip(instructions[delta..].iter())
            .all(|(a, b)| is_matching(a, b));
        println!("0 matches {} = {}", delta, m);
    }
    for i in 0..18 {
        for delta in (0..252).step_by(18) {
            print!("{:?} ", instructions[i + delta]);
        }
        println!();
        println!();
    }
    let mut value_mapping = HashMap::from([
        ('w', String::from("w")),
        ('x', String::from("x")),
        ('y', String::from("y")),
        ('z', String::from("z")),
    ]);
    let mut num_inputs = 0;
    for &instruction in &instructions[0..18] {
        match instruction {
            Inp(a) => {
                value_mapping.insert(a, format!("input[{}]", num_inputs));
                num_inputs += 1;
            }
            Add(a, b) => {
                if value_mapping[&a] == "0" {
                    value_mapping.insert(
                        a,
                        match b {
                            Register(b) => value_mapping[&b].clone(),
                            Number(n) => n.to_string(),
                        },
                    );
                } else {
                    infix_expressioin(&mut value_mapping, a, "+", b)
                }
            }
            Mul(a, b) => {
                if matches!(b, Number(0)) {
                    value_mapping.insert(a, 0.to_string());
                } else {
                    infix_expressioin(&mut value_mapping, a, "*", b);
                }
            }
            Div(a, b) => infix_expressioin(&mut value_mapping, a, "/", b),
            Mod(a, b) => infix_expressioin(&mut value_mapping, a, "%", b),
            Eql(a, b) => infix_expressioin(&mut value_mapping, a, "==", b),
        }
    }
    for (k, v) in value_mapping {
        println!("{} = {}", k, v);
    }
}

fn op(z: i32, input: &[i32]) -> i32 {
    let x = (z % 26) + 11;
    (z / 1) * (25 * (x != input[0]) as i32 + 1) + ((input[0] + 14) * (x != input[0]) as i32)
}
