use std::io;

#[derive(Debug)]
pub enum TemplarError {
    IoError(io::Error),
    ParseError,
}

#[derive(Debug)]
pub struct ParseError {
    pub line_number: usize,
    pub context: Vec<String>, // last few lines
    pub character: Option<u64>,
    pub message: String,
}
