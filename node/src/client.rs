use crate::message::Msg;
use crate::net::TcpConnection;
use crate::result::*;

pub type Id = u64;

pub enum Client {
    Remote(TcpConnection),
}

pub trait Conn {
    fn send(&self, msg: Msg) -> Result<()>;
    fn recv(&self) -> Result<Msg>;
}

impl Conn for Client {
    fn send(&self, msg: Msg) -> Result<()> {
        unimplemented!();
    }

    fn recv(&self) -> Result<Msg> {
        unimplemented!();
    }
}
