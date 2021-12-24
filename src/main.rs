#![allow(dead_code, unused_imports, unused_variables)]

mod common;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;
mod day_19;
mod day_2;
mod day_20;
mod day_21;
mod day_22;
mod day_23;
mod day_24;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;

use crate::common::{make_my_result, MyResult};
use day_24::run_me;
use std::env::args;
use std::error::Error;
use std::fmt::Display;
use std::fs::File;
use std::io::BufReader;

#[derive(Debug)]
struct NotEnoughArgsError;

impl Display for NotEnoughArgsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

impl Error for NotEnoughArgsError {}

fn main() -> MyResult<()> {
    let input_filename = args().nth(1).ok_or(NotEnoughArgsError)?;
    let reader = BufReader::new(File::open(input_filename)?);
    run_me(reader)?;
    Ok(())
}
