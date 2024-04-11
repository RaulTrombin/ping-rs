use std::{error, fmt};

use crate::decoder;

#[derive(Debug)]
pub enum PingError {
    Io(std::io::Error),
    ParseError(decoder::ParseError),
}

impl From<std::io::Error> for PingError {
    fn from(err: std::io::Error) -> PingError {
        PingError::Io(err)
    }
}

impl From<tokio_serial::Error> for PingError {
    fn from(err: tokio_serial::Error) -> PingError {
        PingError::Io(std::io::Error::new(std::io::ErrorKind::Other, err))
    }
}

// impl fmt::Display for PingError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             PingError::Io(err) => write!(f, "IO error: {}", err),
//             PingError::ParseError(err) => write!(f, "Parse error: {:?}", err),
//         }
//     }
// }

// impl error::Error for PingError {}
