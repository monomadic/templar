// mod error;
pub mod parser;
pub mod interpreter;

// pub use error::ParseError;
// pub use parser::run;
// pub use interpreter::run;

// pub type ParseResult<T> = Result<T, ParseError>;
pub type ParseResult<T> = Result<T, Box<dyn std::error::Error>>;

// pub fn parse(i: String) -> ParseResult<Vec<Node>> {
//     let (_, result) = parser::run(i)?;
//     Ok(result)
// }

#[derive(Debug, Clone)]
pub struct Block {
    ident: String,
    properties: Vec<Property>,
    nodes: Vec<Node>,
}

#[derive(Debug, Clone)]
pub enum Property {
    Boolean(bool),
    DottedSymbol(String),
    Float(f64),
    Number(i64),
    QuotedString(String),
    Symbol(String),
}

use std::fmt;
impl fmt::Display for Property {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Property::Boolean(b)        => { if b == &true { write!(f, "true") } else { write!(f, "false") }},
            Property::DottedSymbol(s)   => write!(f, "{}", s),
            Property::Float(n)          => write!(f, "{}", n),
            Property::Number(n)         => write!(f, "{}", n),
            Property::QuotedString(s)   => write!(f, "{}", s),
            Property::Symbol(s)         => write!(f, "{}", s),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Node {
    WhiteSpace,
    Comment(String),
    Block {
        ident: String,
        properties: Vec<Property>, // todo: change to argument object
        children: Vec<Node>,
    },
    Assignment { // rename to property
        ident: String,
        value: Property, // this will end up being its own vec of enums
    },
}

pub struct UnwoundNode {
    ident: String,
    // description: Option<String>,
    properties: Vec<Property>,
    children: Vec<UnwoundNode>,
}
