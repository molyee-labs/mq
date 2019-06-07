#[derive(Debug)]
pub enum Error {
    IoErr(std::io::Error),
    NetErr(crate::net::Error),
}

use Error::*;

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        IoErr(err)
    }
}

impl From<crate::net::Error> for Error {
    fn from(err: crate::net::Error) -> Self {
        NetErr(err)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
