#![allow(dead_code, unused_imports, unused_variables)]

mod common;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;

use day_6::run_me;
use crate::common::{MyResult, make_my_result};


fn main() -> MyResult<()> {
    run_me()?;
    Ok(())
}
