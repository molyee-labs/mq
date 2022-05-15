use crate::result::{Error, Result};
use std::net::SocketAddr;

mod udp;
pub use udp::*;

pub enum Protocol {
    Udp,
}

pub trait ConnectListener {
    fn on_connect(&mut self, addr: SocketAddr);
    fn on_disconnect(&mut self, addr: SocketAddr, reason: Result<()>);
}

pub trait DataHandler {
    fn on_receive(&mut self, addr: SocketAddr, buf: &[u8]);
    fn on_error(&mut self, addr: SocketAddr, err: Error);
}

pub trait Transport {
    fn connect(&mut self, addr: SocketAddr);
    fn disconnect(&mut self, addr: SocketAddr);
    fn send(&self, data: &[u8], addr: SocketAddr);
    fn broadcast(&self, data: &[u8]);
}
