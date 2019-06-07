use crate::result::{
    Error::{IoErr, NetErr},
    Result,
};
use std::cell::RefCell;
use std::fmt::Display;
use std::io::Write;
use std::net::{Shutdown, SocketAddr, TcpStream, ToSocketAddrs};
use std::time::Duration;

#[derive(Debug)]
pub enum Error {
    NoSocketAddrsFound(String),
    NoStream,
}

pub trait Transport: Drop + Sized {
    fn send(&self, buf: &[u8], timeout: Duration) -> Result<()>;
    fn recv(&self, timeout: Duration) -> Result<()>;
}

pub trait Connection: Drop + Sized {
    fn connect(&self, timeout: Duration) -> Result<()>;
    fn close(&self, timeout: Duration) -> Result<()>;
}

pub struct TcpConnection {
    addr: SocketAddr,
    stream: RefCell<Option<TcpStream>>,
}

impl TcpConnection {
    pub fn create<T: ToSocketAddrs + Display>(addr: &T) -> Result<Self> {
        let mut addrs = addr.to_socket_addrs()?;
        let addr = addrs
            .next()
            .ok_or_else(|| Error::NoSocketAddrsFound(format!("{}", addr)))?;
        let conn = TcpConnection {
            addr,
            stream: RefCell::new(None),
        };
        Ok(conn)
    }

    fn update_stream(&self, stream: Option<TcpStream>) -> Result<()> {
        let last = self.stream.replace(stream);
        if let Some(s) = last {
            s.shutdown(Shutdown::Both);
        }
        Ok(())
    }
}

impl Drop for TcpConnection {
    fn drop(&mut self) {
        &self.close(Duration::from_secs(5u64));
    }
}

impl Connection for TcpConnection {
    fn connect(&self, timeout: Duration) -> Result<()> {
        let stream = TcpStream::connect_timeout(&self.addr, timeout)?;
        self.update_stream(Some(stream))
    }

    fn close(&self, timeout: Duration) -> Result<()> {
        self.update_stream(None)
    }
}

impl Transport for TcpConnection {
    fn send(&self, buf: &[u8], timeout: Duration) -> Result<()> {
        let len = buf.len();
        if let Some(stream) = self.stream.borrow_mut().as_mut() {
            stream.write_all(buf).map_err(|e| IoErr(e))
        } else {
            Err(NetErr(Error::NoStream))
        }
    }

    fn recv(&self, timeout: Duration) -> Result<()> {
        unimplemented!()
    }
}
