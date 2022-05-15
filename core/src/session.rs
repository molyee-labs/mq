use core::time::Instant;
use std::net::SocketAddr;

use crate::proto;

pub struct Id(usize);

pub struct Session {
    id: Id,
    stats: Stats,
}

impl Session {
    pub fn new(id: Id) -> Self {
        Self { id, stats }
    }

    pub fn stats(&self) -> &Stats {
        &self.stats
    }
}

pub struct Stats {
    start: Instant,
    msg_in: usize,
    msg_out: usize,
    bytes_in: usize,
    bytes_out: usize,
}

