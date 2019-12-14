use crate::result::{
    Error::{IoErr, NetErr},
    Result,
};
use crate::transport::{Transport, Protocol, ConnectListener};
use crate::transport::UdpTransport;
use crate::message::Message;
use crate::config::HubConf;
use std::sync::Arc;
use std::io::{self, Write};
use std::net::{Shutdown, SocketAddr};
use std::net::ToSocketAddrs;
use std::time::Duration;
use url::HostAndPort;

#[derive(Debug)]
pub enum Error {
    NoSocketAddrsFound(String),
    NoStream,
    IoErr(io::Error)
}

struct Options {
    timeo: Duration
}

impl From<&HubConf> for Options {
    fn from(conf: &HubConf) -> Self {
        let timeo = conf.timeout;
        Self {
            timeo
        }
    }
}

pub struct Connection<'a, H: Hub> {
    last: message::Id, 
    peer: SocketAddr,
    hub: &'a H,
}

impl<H: Hub> Connection<'_, H> {
    pub fn send(&self, data: &[u8]) {

        self.hub.send_to(&self.peer, msg: M)
    }
}

pub enum Reason {
    Closed,
    RemoteClosed,
    Timeout,
}

pub trait Hub {
    fn broadcast<M: Message>(&self, msg: M);
    fn send_to<A: ToSocketAddrs, M: Message>(&self, addr: A, msg: M);
}

pub struct UdpHub {
    trans:
}

impl Hub for UdpHub {

}

impl ConnectListener for Hub {
    fn on_connect(&self, addr: SocketAddr) {
        unimplemented!() 
    }

    fn on_disconnect(&self, addr: SocketAddr, reason: Result<()>) {
        unimplemented!() 
    }
}
