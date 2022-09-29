use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct HttpError {
    message: String,
}

impl HttpError {
    pub fn new(message: String) -> HttpError {
        HttpError { message }
    }
}

impl Display for HttpError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for HttpError {}
