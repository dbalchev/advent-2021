use std::error::Error;


pub type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn make_my_result<T, E: 'static + Error>(original_result: Result<T, E>) -> MyResult<T> {
    Ok(original_result?)
}