use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct TapsError {
    details: String
}

impl TapsError {
    pub fn new(msg: &str) -> TapsError {
        TapsError{details: msg.to_string()}
    }
}

impl fmt::Display for TapsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for TapsError {
    fn description(&self) -> &str {
        &self.details
    }
}
