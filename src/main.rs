#![allow(dead_code, unused_imports, unused_variables)]

mod common;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;

use std::io::BufReader;
use std::fs::File;
use std::error::Error;
use std::fmt::Display;
use std::env::args;
use day_17::run_me;
use crate::common::{MyResult, make_my_result};


#[derive(Debug)]
struct NotEnoughArgsError;

impl Display for NotEnoughArgsError {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> { 
        write!(f, "{:?}", self)
    }
}

impl Error for NotEnoughArgsError{}


fn main() -> MyResult<()> {
    let input_filename = args().nth(1).ok_or(NotEnoughArgsError)?;
    let reader = BufReader::new(File::open(input_filename)?);
    run_me(reader)?;
    Ok(())
}
