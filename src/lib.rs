// mod error;
pub mod parser;
pub mod postprocessor;
mod models;
pub use models::*;

// pub type ParseResult<T> = Result<T, ParseError>;
pub type ParseResult<T> = Result<T, Box<dyn std::error::Error>>;
