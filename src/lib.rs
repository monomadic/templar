pub mod error;
pub mod parser;
pub mod postprocessor;
mod models;
pub use models::*;

// pub type ParseResult<T> = Result<T, ParseError>;
pub type ParseResult<T> = Result<T, Box<dyn std::error::Error>>;
pub type TemplarResult<T> = Result<T, error::TemplarError>;

pub fn parse_str(i: &str) -> TemplarResult<Vec<UnwoundNode>> {
    let (_, nodes) = parser::run(&format!("{}\n", i)).unwrap();
    postprocessor::run(nodes)
}
