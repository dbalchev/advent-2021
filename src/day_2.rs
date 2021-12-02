use std::io::BufRead;
use std::fs::File;
use std::io::BufReader;
use std::io;

fn parse_instruction(line: &str) -> (i64, i64) {
    let components = line.split(' ').collect::<Vec<&str>>();
    match &components[..] {
        ["forward", n] => (n.parse().unwrap(), 0),
        ["down", n] => (0, n.parse().unwrap()),
        ["up", n] => (0, -n.parse::<i64>().unwrap()),
        _ => panic!("No match {}", line),
    }
}

pub fn run_me() -> io::Result<()> {
    let f = BufReader::new(File::open("inputs/day_2.txt")?);
    let mut v_delta = 0i64;
    let mut h_delta = 0i64;
    let mut delta_2 = 0i64;
    for line in f.lines() {
        let line = line.unwrap();
        if line.len() == 0 {
            continue;
        }
        let (h_move, v_move) = parse_instruction(&line);
        v_delta += v_move;
        h_delta += h_move;
        delta_2 += h_move * v_delta;
    }
    println!("{} {}", v_delta * h_delta, delta_2 * h_delta);
    Ok(())
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_parse_instruction() {
        assert_eq!(parse_instruction("forward 2"), (2 , 0));
        assert_eq!(parse_instruction("down 3"), (0 , 3));
        assert_eq!(parse_instruction("up 4"), (0 , -4));
    
    }
}
