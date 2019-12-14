use super::{Transport, ConnectListener, DataHandler};
use crate::result::{Result, Error};
use aio::net::{UdpSocket, ToSocketAddrs};
use std::net::{SocketAddr, Ipv6Addr, Ipv4Addr, IpAddr};
use bytes::{BufMut, BytesMut};

#[derive(PartialEq)]
pub enum UdpMode {
    Listener,
    Multicast,
}

impl Default for UdpMode {
    fn default() -> Self {
        UdpMode::Multicast
    }
}

#[derive(Default, Copy, Clone)]
pub struct Interface(u32);

impl From<Interface> for u32 {
    fn from(i: Interface) -> Self {
        i.0
    }
}

impl From<Interface> for Ipv4Addr {
    fn from(i: Interface) -> Self {
        let b = i.0.to_be_bytes();
        Ipv4Addr::new(b[0], b[1], b[2], b[3])
    }
}

#[derive(Default)]
pub struct UdpOptions {
    ttl: u32,
    mode: UdpMode,
    iface: Interface,
    capacity: usize,
}

pub struct UdpTransport {
    opts: UdpOptions,
    sock: UdpSocket,
    bytes: BytesMut,
}

impl UdpTransport {
    pub async fn new<H: DataHandler + Send, A: ToSocketAddrs>(addr: A, opts: UdpOptions, handler: H) -> Result<Self> {
        let sock = UdpSocket::bind(addr).await?;
        let local_addr = sock.local_addr()
            .map_err(Error::IoErr)?;
        let ip = local_addr.ip();
        sock.set_ttl(opts.ttl);
        if opts.mode == UdpMode::Multicast {
            match ip {
                IpAddr::V4(ipv4) => {
                    let iface = match opts.iface.into() {
                        Ipv4Addr::UNSPECIFIED => ipv4,
                        x => x.into(),
                    };
                    sock.join_multicast_v4(ipv4, iface);
                },
                IpAddr::V6(ipv6) => {
                    sock.join_multicast_v6(&ipv6, opts.iface.into());
                }
            }
        }
        let bytes = BytesMut::with_capacity(opts.capacity);
        let mut transport = Self { opts, sock, bytes };
        aio::executor::spawn(transport.recv(handler));
        Ok(transport)
    }

    pub async fn recv<H: DataHandler + Send>(&mut self, handler: H) -> Result<()> {
        const MIN_FREE_BUF_SIZE: usize = 1024;
        loop {
            let len = self.bytes.len();
            let remaining = self.bytes.remaining_mut();
            let cap = if remaining < MIN_FREE_BUF_SIZE {
                let cap = self.bytes.capacity() + MIN_FREE_BUF_SIZE;
                self.bytes.reserve(cap);
                cap
            } else {
                self.bytes.capacity()
            };
            let buf = self.bytes.bytes_mut();
            let received = self.sock.recv(buf).await;
            match received {
                Ok(size) => self.bytes.set_len(len + size),
                Err(e) => 
            }
        }
    }
}

impl Transport for UdpTransport {
    fn connect(&mut self, addr: SocketAddr) {
        unimplemented!();
    }

    fn disconnect(&mut self, addr: SocketAddr) {
        unimplemented!();
    }

    fn send(&self, data: &[u8], addr: SocketAddr) {
        unimplemented!();
    }

    fn broadcast(&self, data: &[u8]) {
        unimplemented!();
    }
}