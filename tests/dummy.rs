
use std::io::BufRead;
use std::io::BufReader;
use std::io;
use std::fs::File;
#[test]
fn test_read() -> io::Result<()>{
    let f = BufReader::new(File::open("tests/dummy-input.txt")?);
    let numbers: Vec<i32> = f.lines().map(|x| x.unwrap().parse().unwrap()).collect();
    assert_eq!(numbers.into_iter().sum::<i32>(), 7i32);
    Ok(())
}