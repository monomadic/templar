use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Property {
    ArgumentIndex(usize), // rename to argument reference
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
            Property::ArgumentIndex (n) => write!(f, "{}", n),
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
    Overlay(Overlay),
    Block {
        ident: String,
        attributes: Vec<Property>,
        children: Vec<Node>,
    },
    Assignment { // rename to Property
        ident: String,
        value: Property, // this will end up being its own vec of enums
    },
    AnonymousProperty(Property), // is this valid?
}

#[derive(Debug, Clone)]
pub struct Block {
    pub ident: String,
    // description: Option<String>,
    pub attributes: Vec<Property>,
    pub properties: HashMap<String, Property>,
    pub children: Vec<Node>,
}

// pub struct Argument {
//     ident: String,
//     // type: 
//     default: Option<Property>,
// }

// todo: wrap up into Block
#[derive(Debug, Clone)]
pub struct UnwoundNode {
    pub ident: String,
    // description: Option<String>,
    pub attributes: Vec<Property>, // todo: remove this, they should be wound into properties.
    pub properties: HashMap<String, Property>,
    pub children: Vec<UnwoundNode>,
}

impl UnwoundNode {
    pub fn get_local(&self, ident: &str) -> Option<Property> {
        self.properties.get(ident).map(|p|p.clone())
    }

    pub fn display(&self, indent: usize) -> String {
        let properties = self.properties.iter().map(|(k,v)| {
            format!("\n{: >i$}.{} {:?}", "", k, v, i=indent+1)
        }).collect::<Vec<String>>().join("");

        let children = self.children.iter().map(|c| c.display(indent + 1)).collect::<Vec<String>>().join("\n");

        let ident = if self.ident == "_TEXT".to_string() {
            format!("{:?}", self.get_local("text").unwrap_or(Property::QuotedString("".to_string())))
        } else {
            self.ident.clone()
        };

        format!("{: >i$}{}{}\n{}", "", ident, properties, children, i=indent)
    }
}

// todo: rename to overlay
#[derive(Debug, Clone)]
pub struct Overlay {
    pub ident: String,
    pub output: String,
    pub arguments: Vec<String>,
    pub children: Vec<Node>,
}
