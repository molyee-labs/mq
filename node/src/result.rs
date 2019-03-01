pub enum Error {
    Io(std::io::Error),
    NoSocketAddrsFound(String),
}

use Error::*;

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Io(err)
    }
}

pub type Result<T> = std::result::Result<T, Error>;