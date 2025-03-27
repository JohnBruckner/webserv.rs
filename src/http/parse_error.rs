use core::fmt;
use std::error::Error;

#[derive(Debug)]
pub struct HttpParseError {
    message: String
}

impl HttpParseError {
    pub fn new(msg: &str) -> HttpParseError{
        HttpParseError {message: msg.to_string()}
    }
}

impl fmt::Display for HttpParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for HttpParseError {
    fn description(&self) -> &str {
        &self.message
    } 
}