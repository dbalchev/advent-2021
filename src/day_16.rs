use crate::MyResult;
use std::{collections::VecDeque, fmt::Result, io::BufRead, iter::from_fn};

#[derive(Debug)]
enum Content {
    Literal(i64),
    Subpackets(Vec<Packet>),
}

#[derive(Debug)]
struct Packet {
    version: i32,
    type_id: i32,
    content: Content,
}

type BinaryStream = VecDeque<char>;

fn read_number(binary_stream: &mut BinaryStream, n_bits: usize) -> MyResult<i64> {
    if binary_stream.len() < n_bits {
        return Err(format!(
            "expected {} bits but got only {}",
            n_bits,
            binary_stream.len()
        ))?;
    }
    Ok(i64::from_str_radix(
        &binary_stream.drain(..n_bits).collect::<String>(),
        2,
    )?)
}

fn read_literal(binary_stream: &mut BinaryStream) -> MyResult<Content> {
    let mut value = 0;
    let mut has_more = 1;
    while has_more != 0 {
        has_more = read_number(binary_stream, 1)?;
        value = value * 16 + read_number(binary_stream, 4)?;
    }
    Ok(Content::Literal(value))
}

fn read_packet(binary_stream: &mut BinaryStream) -> MyResult<Packet> {
    let version = read_number(binary_stream, 3)? as i32;
    let type_id = read_number(binary_stream, 3)? as i32;
    let content = if type_id == 4 {
        read_literal(binary_stream)?
    } else {
        let length_id = read_number(binary_stream, 1)?;
        Content::Subpackets(if length_id == 0 {
            let inner_len = read_number(binary_stream, 15)? as usize;
            let mut inner_stream = binary_stream.drain(..inner_len).collect::<VecDeque<_>>();
            from_fn(|| read_packet(&mut inner_stream).ok()).collect()
        } else {
            let num_packets = read_number(binary_stream, 11)?;
            (0..num_packets)
                .map(|_| read_packet(binary_stream))
                .collect::<MyResult<Vec<_>>>()?
        })
    };
    return Ok(Packet {
        version,
        type_id,
        content,
    });
}

fn sum_version(packet: &Packet) -> i32 {
    packet.version
        + match packet.content {
            Content::Literal(..) => 0,
            Content::Subpackets(ref subpackets) => subpackets.iter().map(sum_version).sum(),
        }
}

fn eval(packet: &Packet) -> i64 {
    match packet.content {
        Content::Literal(x) => x,
        Content::Subpackets(ref items) => match packet.type_id {
            0 => items.iter().map(eval).sum(),
            1 => items.iter().map(eval).product(),
            2 => items.iter().map(eval).min().unwrap(),
            3 => items.iter().map(eval).max().unwrap(),
            5 => (eval(&items[0]) > eval(&items[1])) as i64,
            6 => (eval(&items[0]) < eval(&items[1])) as i64,
            7 => (eval(&items[0]) == eval(&items[1])) as i64,
            ti => panic!("not implemented {}", ti),
        },
    }
}

pub fn run_me(reader: impl BufRead) -> MyResult<()> {
    let hex_input = reader.lines().next().ok_or(format!("empty input"))??;
    let mut binary_input = hex_input
        .chars()
        .flat_map(|c| {
            format!("{:04b}", i32::from_str_radix(&String::from(c), 16).unwrap())
                .chars()
                .collect::<Vec<_>>()
        })
        .collect::<BinaryStream>();

    // println!("{:?}", binary_input);
    let packet = read_packet(&mut binary_input)?;
    // println!("{:?}", packet);
    println!("Task 1: {}", sum_version(&packet));
    println!("Task 2: {}", eval(&packet));

    Ok(())
}
