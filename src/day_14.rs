use crate::MyResult;
use std::collections::HashMap;
use std::io::BufRead;

type PolyPair = (u8, u8);
type CountMap = HashMap<PolyPair, u64>;
type TransformMap = HashMap<PolyPair, [PolyPair; 2]>;

fn do_step(transform_map: &TransformMap, count_map: CountMap) -> CountMap {
    let mut result = CountMap::new();

    for (pair, count) in count_map {
        if let Some([new_pair_1, new_pair_2]) = transform_map.get(&pair) {
            *result.entry(*new_pair_1).or_default() += count;
            *result.entry(*new_pair_2).or_default() += count;
        } else {
            *result.entry(pair).or_default() += count;
        }
    }
    result
}

fn compute_char_counts(count_map: CountMap, first_char: u8, last_char: u8) -> [u64; 128] {
    let mut result = [0; 128];

    result[first_char as usize] += 1;
    result[last_char as usize] += 1;

    for ((char_1, char_2), count) in count_map {
        result[char_1 as usize] += count;
        result[char_2 as usize] += count;
    }
    for c in &mut result {
        assert_eq!(*c % 2, 0);
        *c /= 2;
    }
    result
}

pub fn run_me(reader: impl BufRead) -> MyResult<()> {
    let mut lines = reader.lines();
    let template = lines.next().ok_or(format!("no first line"))??.into_bytes();

    assert_eq!(lines.next().ok_or(format!("no second line"))??.len(), 0);

    let transform_map = lines
        .map(|line| {
            let line = line?;
            let (starting_point, result) = line
                .split_once(" -> ")
                .ok_or(format!("Cannot split {}", line))?;
            let starting_point = starting_point.as_bytes();
            let result = result.as_bytes();
            assert_eq!(starting_point.len(), 2);
            assert_eq!(result.len(), 1);
            Ok((
                (starting_point[0], starting_point[1]),
                [
                    (starting_point[0], result[0]),
                    (result[0], starting_point[1]),
                ],
            ))
        })
        .collect::<MyResult<TransformMap>>()?;
    let mut initial_count_map = CountMap::new();
    for (&c1, &c2) in template[..template.len() - 1]
        .iter()
        .zip(template[1..].iter())
    {
        *initial_count_map.entry((c1, c2)).or_default() += 1;
    }
    let mut count_map = initial_count_map.clone();
    for _ in 0..10 {
        count_map = do_step(&transform_map, count_map);
    }
    let char_counts = compute_char_counts(count_map, template[0], template[template.len() - 1]);
    let &max_count = char_counts.iter().max().ok_or(format!("Empty counts"))?;
    let &min_count = char_counts
        .iter()
        .filter(|x| **x > 0)
        .min()
        .ok_or(format!("Empty counts"))?;
    println!("{} {}", max_count, min_count);

    println!("Task 1: {}", max_count - min_count);

    Ok(())
}
