use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum TemplarError {
    ParseError,
    LocalNotFound(String),
    InternalError(String),
    GenericError
}

impl Error for TemplarError {}

impl fmt::Display for TemplarError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TemplarError::ParseError            => write!(f, "ParseError"),

            TemplarError::InternalError(ref e)            => write!(f, "Internal Error: {}", e),
            TemplarError::LocalNotFound(ref e)            => write!(f, "Local not found Error: {}", e),
             
            // TemplarError::InvalidCharacter(ref c) => write!(f, "InvalidCharacter: {}: expected `{}`, got `{}`",
            //                                             c.position, c.expected, c.got),
            // TemplarError::InvalidRowLength(ref c) => write!(f, "InvalidRowLength: {}: expected `{}` element{}, got `{}`",
            //                                             c.position, c.nb_elements_expected,
            //                                             if c.nb_elements_expected > 1 { "s" } else { "" }, c.nb_elements_got),
            TemplarError::GenericError            => write!(f, "GenericError")
        }
    }
}