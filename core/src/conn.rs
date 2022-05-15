use std::{collections::HashMap, net::{SocketAddr, ToSocketAddrs}};

use crate::{messages::Message, proto::Protocol, session::{self, Session}};

pub trait Connection {
    fn session(&self) -> &Session;
    fn is_permanent(&self) -> bool;
    async fn send(&self, buf: &[u8]) -> Result<usize>;
    async fn recv(&self, buf: &mut [u8]) -> Result<usize>;
    fn close(self) -> Result<Session>;
}

pub trait Controller {
    type Cid;
    type Conn: Connection;
    async fn connect<A: ToSocketAddrs>(&mut self, addr: A) -> Result<Cid>;
    async fn bind<A: ToSocketAddrs>(&mut self, addr: A) -> Result<()>;
    async fn broadcast(&self, buf: &[u8]) -> Result<usize>;
    async fn send(&self, buf: &[u8], target: Self::Cid) -> Result<usize>;
    async fn recv(&self, buf: &mut [u8]) -> Result<(usize, Self::Cid)>;
    fn close(&mut self, cid: Self::Cid) -> Result<()>;
    fn get(&self, cid: Self::Cid) -> Option<&Self::Conn>;
}

pub trait Listener {
    fn on_connect(&mut self, conn: &Connection);
    fn on_error(&mut self, conn: &Connection, err: Error);
    fn on_disconnect(&mut self, conn: Connection, reason: Reason);
}