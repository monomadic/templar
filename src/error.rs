use std::io;

#[derive(Debug)]
enum TemplarError {
    IoError(io::Error),
    ParseError,
}

use std::fmt;
impl fmt::Display for TemplarError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TemplarError::IoError(ref e) => write!(f, "IO error: {}", e),
            TemplarError::ParseError => write!(f, "Parse error"),
        }
    }
}

use std::error;
impl error::Error for TemplarError {
    fn description(&self) -> &str {
        match *self {
            TemplarError::IoError(ref e) => e.description(),
            TemplarError::ParseError => "parse error",
        }
    }

    // fn cause(&self) -> Option<&error::Error> {
    //     match *self {
    //         TemplarError::IoError(ref e) => Some(e),
    //         TemplarError::ParseError => None,
    //     }
    // }
}