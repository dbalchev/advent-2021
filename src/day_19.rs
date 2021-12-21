use std::{collections::HashSet, io::BufRead};

use crate::common::MyResult;
use itertools::Itertools;
use ndarray::{array, stack, Array1, Array2, Axis, Zip};
use std::str::FromStr;

#[derive(Debug)]
struct Alignment {
    transformed: Array2<i32>,
    delta: Array1<i32>,
}

fn compute_orientations() -> Vec<Array2<i32>> {
    let unit_vectors = [array![1, 0, 0], array![0, 1, 0], array![0, 0, 1]];
    unit_vectors
        .iter()
        .combinations(2)
        .flat_map(|v| {
            [
                (v[0].clone(), v[1].clone()),
                (v[0].clone(), -v[1].clone()),
                (-v[0].clone(), v[1].clone()),
                (-v[0].clone(), -v[1].clone()),
            ]
        })
        .flat_map(|(a, b)| [(a.clone(), b.clone()), (b, a)])
        .map(|(a, b)| {
            let c = array![
                a[1] * b[2] - a[2] * b[1],
                a[2] * b[0] - a[0] * b[2],
                a[0] * b[1] - a[1] * b[0],
            ];
            stack![Axis(0), a, b, c]
        })
        .collect_vec()
}

fn find_alignment(
    reference: &Array2<i32>,
    reference_delta: &Array1<i32>,
    to_transform: &Array2<i32>,
    orientations: &[Array2<i32>],
) -> Option<Alignment> {
    let reference_set = reference
        .axis_iter(Axis(0))
        .map(|a| a.into_owned())
        .collect::<HashSet<_>>();
    for orientation in orientations {
        let oriented = to_transform.dot(orientation);
        for r in reference.axis_iter(Axis(0)) {
            'o_loop: for o in oriented.axis_iter(Axis(0)) {
                let delta = &r - &o;

                let mut overlap_size = 0;
                for t in oriented.axis_iter(Axis(0)) {
                    let t = &t + &delta;
                    if (&t - reference_delta).mapv(i32::abs).iter().max().unwrap() >= &1_000 {
                        continue;
                    }
                    if !reference_set.contains(&t) {
                        continue 'o_loop;
                    }
                    overlap_size += 1;
                }

                if overlap_size >= 12 {
                    return Some(Alignment {
                        transformed: &oriented + &delta,
                        delta,
                    });
                }
            }
        }
    }
    None
}

pub fn run_me(read: impl BufRead) -> MyResult<()> {
    let mut peekable_lines = read.lines().peekable();
    let mut scanners = Vec::new();

    loop {
        let header = peekable_lines.next().ok_or("no header")??;
        assert!(
            header.starts_with("---") && header.ends_with("---"),
            "'{}'",
            header
        );
        let scanner_data = peekable_lines
            .peeking_take_while(|x| match x {
                Ok(s) => s.len() > 0,
                Err(_) => true,
            })
            .map(|line| {
                Ok(Array1::from_vec(
                    line?
                        .split(",")
                        .map(i32::from_str)
                        .collect::<Result<Vec<_>, _>>()?,
                ))
            })
            .collect::<MyResult<Vec<Array1<_>>>>()?;
        scanners.push(stack(
            Axis(0),
            &scanner_data.iter().map(Array1::view).collect::<Vec<_>>(),
        )?);

        let empty_line = match peekable_lines.next() {
            Some(r) => r?,
            None => break,
        };
        assert_eq!(empty_line.len(), 0, "'{}'", empty_line);
        if scanner_data.len() == 0 {
            break;
        }
    }
    // println!("{:?}", scanners);
    let orientations = compute_orientations();
    scanners.reverse();
    let mut aligned = vec![Alignment {
        transformed: scanners.pop().unwrap(),
        delta: array![0, 0, 0],
    }];
    while scanners.len() > 0 {
        let mut found = false;
        for i in 0..scanners.len() {
            let mut new_a = None;
            for Alignment { transformed, delta } in &aligned {
                if let Some(a) = find_alignment(transformed, delta, &scanners[i], &orientations) {
                    new_a = Some(a);
                    break;
                }
            }
            if let Some(a) = new_a {
                aligned.push(a);
                scanners.remove(i);
                found = true;
                println!("found");
                break;
            }
        }
        assert!(found);
    }
    let beakon_set = aligned
        .into_iter()
        .flat_map(|Alignment { transformed, .. }| {
            transformed
                .axis_iter(Axis(0))
                .map(|a| a.into_owned())
                .collect_vec()
        })
        .collect::<HashSet<_>>();
    println!("Task 1: {}", beakon_set.len());
    Ok(())
}
