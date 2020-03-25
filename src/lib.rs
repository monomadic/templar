mod error;
mod parse;

pub use error::ParseError;
pub use parse::parse;

pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Debug, Clone)]
pub enum Node {
    Call {
        id: String,
        args: Vec<String>,
        children: Vec<Node>,
    },
    Declaration(String),
}
