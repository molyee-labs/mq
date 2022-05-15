use super::{Transport, DataHandler};
use crate::result::{Result, Error};
use aio::net::{UdpSocket, ToSocketAddrs};
use std::net::{SocketAddr, Ipv4Addr, IpAddr};
use std::collections::HashMap;
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

pub struct UdpOptions {
    ttl: u8,
    mode: UdpMode,
    iface: Interface,
    capacity: usize,
    max_capacity: usize,
}

impl UdpOptions {
    pub fn new(ttl: u8, mode: UdpMode, iface: Interface, capacity: usize, max_capacity: usize) -> Self {
        Self { ttl, mode, iface, capacity, max_capacity }
    }
}

impl Default for UdpOptions {
    fn default() -> Self {
        Self::new(8, UdpMode::Multicast, 0.into(), 65535, 10485760)
    }
}

pub struct UdpTransport {
    opts: UdpOptions,
    sock: Option<UdpSocket>,
    data: BytesMut,
}

impl UdpTransport {
    pub fn new(opts: UdpOptions) -> Self {
        Self {
            opts,
            sock: None,
            data: BytesMut::with_capacity(opts.capacity),
        }
    }

    pub async fn bind<A: ToSocketAddrs>(self, addr: A) -> Result<()> {
        info!("Binding to Udp socket addr '{}'", addr);
        let sock = UdpSocket::bind(addr).await?;
        let local_addr = sock.local_addr()
            .map_err(Error::IoErr)?;
        let ip = local_addr.ip();
        sock.set_ttl(self.opts.ttl);
        if self.opts.mode == UdpMode::Multicast {
            let iface = self.opts.iface;
            match ip {
                IpAddr::V4(ipv4) => {
                    let iface = match iface.into() {
                        Ipv4Addr::UNSPECIFIED => ipv4,
                        x => x.into(),
                    };
                    sock.join_multicast_v4(ipv4, iface);
                },
                IpAddr::V6(ipv6) => {
                    sock.join_multicast_v6(&ipv6, iface.into());
                }
            }
        }
        self.sock = Some(sock);
        aio::spawn(self.recv());
        Ok(())
    }

    pub fn listen(&mut self, addr: SocketAddr, handler: &dyn DataHandler) {
        info!("Add UdpSocket '{}' handler '{}'", addr, handler);
        self.handlers.insert(addr, handler);
    } 

    pub fn unlisten(&mut self, addr: SocketAddr) {
        let handler = self.handlers.remove(addr);
        info!("Remove UdpSocket '{}' handler '{}'", addr, handler);
    }

    async fn recv(self) -> Result<()> {
        const MIN_FREE_BUF_SIZE: usize = 1024;
        loop {
            if self.data.capacity() >= self.opts.max_capacity {
                todo!() 
            }
            let len = self.data.len();
            let remaining = self.data.remaining_mut();
            let cap = if remaining < MIN_FREE_BUF_SIZE {
                let cap = self.data.capacity() + MIN_FREE_BUF_SIZE;
                self.data.reserve(cap);
                cap
            } else {
                self.data.capacity()
            };
            let buf = self.data.bytes_mut();
            let received = self.sock?.recv(buf).await?;
            self.data.set_len(len + received);
            debug!("Received {} + {} bytes into UdpSocket '{}'", len, received, local_addr);
            self.handle_data();
        }
    }

    fn handle_data(&mut self) {

    }
}

impl Transport for UdpTransport {
    fn connect(&mut self, addr: SocketAddr) {
        todo!()
    }

    fn disconnect(&mut self, addr: SocketAddr) {
        todo!()
    }

    fn send(&self, data: &[u8], addr: SocketAddr) {
        todo!()
    }

    fn broadcast(&self, data: &[u8]) {
        todo!()
    }
}