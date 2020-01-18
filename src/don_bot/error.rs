use std::error::Error;
use std::fmt;

pub type DonBotResult<T> = Result<T, Box<Error>>;

#[derive(Debug)]
pub struct DBError {
    details : String
}

impl DBError{
    pub fn new (msg : &str) -> DBError {
        DBError { details : msg.to_string()}
    }
}

impl fmt::Display for DBError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for DBError {
    fn description(&self) -> &str {
        &self.details
    }
}
