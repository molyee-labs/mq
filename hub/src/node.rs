use crate::net::Connection;
use crate::config::NodeConf;
use std::sync::Arc;
use std::hash::Hash;

#[derive(Eq, PartialEq, Clone, Hash)]
pub struct Id(uuid::Uuid);

impl Id {
    pub fn new() -> Self {
        unimplemented!()
    }
}

struct Options {

}

pub struct Node {
    id: Id,
    opts: Options,
    conn: Connection,
}

unsafe impl Sync for Node {}

impl Node {
    pub fn new(id: Id, opts: Options, conn: Connection) -> Self {
        Self {
            id,
            opts,
            conn
        }
    }

    pub fn id(&self) -> &Id {
        &self.id
    }

    pub fn conn(&self) -> &Connection {
        &self.conn
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        unimplemented!()
    }
}
