mod error;
mod parse;

pub use error::ParseError;
pub use parse::parse;

// pub type ParseResult<T> = Result<T, ParseError>;
pub type ParseResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Clone)]
pub struct Block {
    ident: String,
    properties: Vec<Property>,
    nodes: Vec<Node>,
}

#[derive(Debug, Clone)]
pub enum Property {
    Symbol(String),
    QuotedString(String),
}

#[derive(Debug, Clone)]
pub enum Node {
    WhiteSpace,
    Comment(String),
    Block {
        ident: String,
        properties: Vec<Property>,
        children: Vec<Node>,
    },
    Assignment {
        ident: String,
        value: Property, // this will end up being its own vec of enums
    },
}
