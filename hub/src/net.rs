use crate::result::{
    Error::{IoErr, NetErr},
    Result,
};
use crate::transport::{Protocol, ConnectListener, DataHandler};
use crate::transport::{UdpTransport, UdpOptions};
use crate::message::Message;
use crate::config::HubConf;
use std::io::{self, Write};
use std::net::{Shutdown, SocketAddr};
use std::net::ToSocketAddrs;
use std::time::Duration;
use std::collections::HashMap;
use url::HostAndPort;
use uuid::Uuid;

#[derive(Debug)]
pub enum Error {
    NoSocketAddrsFound(String),
    NoStream,
    IoErr(io::Error)
}

struct Options {
    timeout: Duration
}

impl From<&HubConf> for Options {
    fn from(conf: &HubConf) -> Self {
        let timeout = conf.timeout;
        Self {
            timeout
        }
    }
}

pub trait Sender {
    fn broadcast<M: Message>(&self, msg: M);
    fn send_to<A: ToSocketAddrs, M: Message>(&self, addr: A, msg: M);
}

pub struct Connection {
    id: Uuid,
    tx: mpsc::Sender,
    persist: bool,
    opts: Options,
}

impl Sender for Connection {
    fn broadcast<M: Message>(&self, msg: M) {
        unimplemented!()
    }

    fn send_to<A: ToSocketAddrs, M: Message>(&self, addr: A, msg: M) {
        unimplemented!()
    }
}

pub trait HubListener {
    fn on_connect(&self, conn: &Connection);
    fn on_disconnect(&self, conn: &Connection);
    fn on_receive(&self, from: &Connection, data: &[u8]);
    fn on_error(&self, from: &Connection, error: &Error);
}

pub struct UdpHub {
    opts: Options,
    trans: UdpTransport,
    listeners: Vec<dyn HubListener>,
    connects: HashMap<SocketAddr, Connection>,
    rx: mpsc::Receiver,
}

impl UdpHub {
    fn new(conf: &HubConf) -> Self {
        Self {
            opts: conf.into(),
            trans: UdpTransport::new(UdpOptions::default()),
            listeners: vec![],
            connects: HashMap::new(),
        }
    }

    pub fn listen(&mut self, listener: &dyn HubListener) {
        info!("Start listening UdpHub {}", listener);
        self.listeners.push(listener);
    }

    pub fn unlisten(&mut self, listener: &dyn HubListener) {
        info!("Stop listening UdpHub {}", listener);
        self.listeners.remove_item(listener);
    }

    fn handle_data(&mut self, conn: &Connection, buf: &[u8]) {
        for listener in self.listeners.iter() {
            listener.on_receive(conn, buf);
        }
        Ok(())
    }

    fn handle_error(&mut self, conn: &Connection, err: Error) {
        for listener in self.listeners.iter() {
            listener.on_error(conn, err);
        }
        Ok(())
    }

    fn handle_incoming_conn(&mut self, addr: SocketAddr, buf: &[u8]) {
        todo!()
    }

    pub fn run() {
        todo!()
    }
}

impl ConnectListener for UdpHub {
    fn on_connect(&mut self, addr: SocketAddr) {
        let conn = Connection::default();
        self.connects.push(addr, conn);
    }

    fn on_disconnect(&mut self, addr: SocketAddr, reason: Result<()>) {
        match self.connects.remove(addr) {
            
        }
    }
}

impl DataHandler for UdpHub {
    fn on_receive(&mut self, addr: SocketAddr, buf: &[u8]) {
        if let Some(conn) = self.connects.get(addr) {
            self.handle_data(conn, buf);
        } else {
            self.handle_incoming_conn(addr, buf);
        }
    }

    fn on_error(&mut self, addr: SocketAddr, err: Error) {
        if let Some(conn) = self.connects.get(addr) {
            self.handle_error(conn, err);
        } else {
            self.on_disconnect(addr, Err(err));
        }
    }
}
