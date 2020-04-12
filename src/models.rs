use std::collections::HashMap;

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
    Comment(String),
    FunctionDeclaration {
        ident: String,
        arguments: Vec<String>,
        children: Vec<Node>,
    },
    Block { // rename to function call?
        ident: String,
        properties: Vec<Property>, // todo: change to arguments
        children: Vec<Node>,
    },
    Assignment { // rename to PropertyAssignment
        ident: String,
        value: Property, // this will end up being its own vec of enums
    },
    AnonymousProperty(Property),
}

pub struct Argument {
    ident: String,
    // type: 
    default: Option<Property>,
}


#[derive(Debug, Clone)]
pub struct UnwoundNode {
    pub ident: String,
    // description: Option<String>,
    pub properties: Vec<Property>,
    pub locals: HashMap<String, Property>,
    pub children: Vec<UnwoundNode>,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub arguments: Vec<String>,
    pub children: Vec<Node>,
}
