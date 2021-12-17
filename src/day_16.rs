
use std::io::BufRead;
use crate::MyResult;

enum Content {
    Literal(i32),
    Subpackets(Vec<Packet>),
}

struct Packet {
    version: i32,
    type_id: i32,
    content: Content,
}

pub fn run_me(reader: impl BufRead) -> MyResult<()> {
    let hex_input = reader.lines().next().ok_or(format!("empty input"))??;
    let mut binary_input = String::new();
    for c in hex_input.chars() {
        binary_input.push_str(&format!("{:04b}", i32::from_str_radix(&String::from(c), 16)?));
    }
    
    println!("{}", binary_input);
    Ok(())
}