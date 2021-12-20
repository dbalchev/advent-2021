use crate::MyResult;
use itertools::iterate;
use itertools::Itertools;
use ndarray::Dimension;
use ndarray::{s, stack, Array1, Array2, Axis, Zip};
use std::io::{BufRead, Result as IOResult};

fn enchance(image: Array2<u8>, lookup: &[u8], padding: u8) -> (Array2<u8>, u8) {
    let (height, width) = image.raw_dim().into_pattern();
    let mut padded_image = Array2::from_elem([height + 4, width + 4], padding);
    padded_image
        .slice_mut(s![2..(height + 2), 2..(width + 2)])
        .assign(&image);
    assert!(padded_image.is_standard_layout());
    (
        Zip::from(padded_image.windows([3, 3])).map_collect(|x| {
            let num = usize::from_str_radix(
                &x.axis_iter(Axis(0))
                    .flat_map(|x| x)
                    .map(|&c| match c {
                        b'#' => '1',
                        b'.' => '0',
                        _ => panic!("unexpected byte {}", c),
                    })
                    .collect::<String>(),
                2,
            )
            .unwrap();
            lookup[num]
        }),
        lookup[if padding == b'.' { 0 } else { 511 }],
    )
}

pub fn run_me(reader: impl BufRead) -> MyResult<()> {
    let mut lines = reader.lines();

    let enchancement_lookup = lines.next().ok_or("no first line")??.into_bytes();

    let empty_line = lines.next().ok_or("no second line")??;

    let input_image = stack(
        Axis(0),
        &lines
            .map_ok(|x| Array1::from_vec(x.into_bytes()))
            .collect::<IOResult<Vec<_>>>()?
            .iter()
            .map(Array1::view)
            .collect::<Vec<_>>(),
    )?;
    let enchanced_2 = {
        let mut x = input_image.clone();
        let mut padding = b'.';
        for _ in 0..2 {
            let (new_x, new_padding) = enchance(x, &enchancement_lookup, padding);
            x = new_x;
            padding = new_padding;
        }
        x
    };
    println!(
        "Task 1: {}",
        enchanced_2
            .iter()
            .map(|&x| match x {
                b'#' => 1i32,
                b'.' => 0i32,
                _ => panic!("unexpected byte {}", x),
            })
            .sum::<i32>()
    );
    let enchanced_50 = {
        let mut x = input_image.clone();
        let mut padding = b'.';
        for _ in 0..50 {
            let (new_x, new_padding) = enchance(x, &enchancement_lookup, padding);
            x = new_x;
            padding = new_padding;
        }
        x
    };
    println!(
        "Task 2: {}",
        enchanced_50
            .iter()
            .map(|&x| match x {
                b'#' => 1i32,
                b'.' => 0i32,
                _ => panic!("unexpected byte {}", x),
            })
            .sum::<i32>()
    );
    Ok(())
}
