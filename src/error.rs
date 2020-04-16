use std::io;

#[derive(Debug)]
pub enum TemplarError {
    IoError(io::Error),
    ParseError,
    LocalNotFound(String)
}

impl Error for TemplarError {}
