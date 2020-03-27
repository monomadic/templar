mod error;
mod parse;

pub use error::ParseError;
pub use parse::parse;

// pub type ParseResult<T> = Result<T, ParseError>;
pub type ParseResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Clone)]
pub struct Block {
    ident: String,
    properties: Vec<Parameter>,
    nodes: Vec<Node>,
}

#[derive(Debug, Clone)]
pub enum Parameter {
    Symbol(String),
    QuotedString(String),
}

#[derive(Debug, Clone)]
pub enum Node {
    Call {
        ident: String,
        properties: Vec<String>,
        children: Vec<Node>,
    },
    Assignment {
        ident: String,
        value: String, // this will end up being its own vec of enums
    },
}
