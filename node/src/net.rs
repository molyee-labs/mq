use std::net::{SocketAddr, TcpStream, ToSocketAddrs};
use std::fmt::Display;
use std::time::Duration;
use std::cell::RefCell;
use crate::result::*;

pub struct TcpConnection {
    addr: SocketAddr,
    stream: RefCell<TcpStream>,
}

impl TcpConnection {
    pub fn connect<T: ToSocketAddrs + Display>(addr: &T, timeout: Duration) -> Result<Self> {
        let addrs = addr.to_socket_addrs()?;
        let addr = addrs.next().ok_or_else(|| Error::NoSocketAddrsFound(format!("{}", addr)))?;
        let stream = TcpStream::connect(addr)?;
        let stream = RefCell::from(stream);
        Ok(TcpConnection { addr, stream })
    }
}

pub trait Transport : Drop + Sized {
    fn send(&self, buf: &[u8], timeout: Duration) -> Result<()>;
    fn recv(&self, timeout: Duration) -> Result<()>;
}

pub trait Protocol<T: Transport> : Sized {
    fn trans(&self) -> &T;
}

pub trait Connection : Drop + Sized {
    fn connect(&self, timeout: Duration) -> Result<()>;
    fn close(&self, timeout: Duration) -> Result<()>;
}

impl Drop for TcpConnection {
    fn drop(&mut self) {
        
    }
}

impl<T: Transport, P: Protocol<T>> Connection<T, P> for TcpConnection {


    fn connect(&self, timeout: Duration) -> Result<()> {
        unimplemented!()
    }

    fn close(&self, timeout: Duration) -> Result<()> {
        unimplemented!()
    }
}

impl Transport for TcpConnection {
    fn send(&self, buf: &[u8], timeout: Duration) -> Result<usize> {
        let len = buf.len();
        self.stream.get_mut().write_all(buf)
    }

    fn (&self, )
}