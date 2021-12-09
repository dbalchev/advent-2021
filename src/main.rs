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

use day_9::run_me;
use crate::common::{MyResult, make_my_result};


fn main() -> MyResult<()> {
    run_me()?;
    Ok(())
}
