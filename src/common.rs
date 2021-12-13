use crate::Display;
use std::error::Error;


pub type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct GeneralError(pub String);

impl Display for GeneralError {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> { 
        write!(f, "{:?}", self)
    }
}

impl Error for GeneralError{}


pub fn make_my_result<T, E: 'static + Error>(original_result: Result<T, E>) -> MyResult<T> {
    Ok(original_result?)
}