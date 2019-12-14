use crate::result::{Error, Result};
use std::net::{ToSocketAddrs, SocketAddr};

mod udp;
pub use udp::*;

pub enum Protocol {
    Udp,
}

pub trait ConnectListener {
    fn on_connect(&self, addr: SocketAddr);
    fn on_disconnect(&self, addr: SocketAddr, reason: Result<()>);
}

pub trait DataHandler {
    fn on_recieve(&self, addr: SocketAddr, buf: &[u8]) -> Result<()>;
    fn on_error(&self, addr: SocketAddr, err: Error);
}

pub trait Transport {
    fn bind(&mut self, addr: SocketAddr);
    fn connect(&mut self, addr: SocketAddr);
    fn disconnect(&mut self, addr: SocketAddr);
    fn send(&self, data: &[u8], addr: SocketAddr);
    fn broadcast(&self, data: &[u8]);
}
