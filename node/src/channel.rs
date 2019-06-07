use crate::client;
use std::time;
use std::vec::Vec;
use shared;

struct Times {
    created: time::Instant,
    last_msg: time::Duration,
}

struct Subscription {
    client: client::Id,
    times: time::Duration, 
}

struct Statistics {
    msg_in: u64,
    msg_out: u64,
    bytes_in: u64,
    bytes_out: u64,
}

struct State {

}

struct Stream {
    state: State,
    stats: Statistics,
    time: Times,
    subs: Vec<Subscription>,
}

pub struct Path(String);

pub struct Channel {
    stream: shared::LinkMut<Option<Stream>>,
}

unsafe impl Sync for Channel { }
