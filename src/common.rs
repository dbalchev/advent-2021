use std::error::Error;


pub type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn make_my_result<T, E: 'static + Error>(original_result: Result<T, E>) -> MyResult<T> {
    match original_result {
        Ok(x) => Ok(x),
        Err(e) => Err(Box::new(e)),
    }
}