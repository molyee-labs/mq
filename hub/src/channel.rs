use std::time::{Instant, Duration};

struct Times {
    created: Instant,
    last_msg: Duration,
}

struct Stats {
    msg_in: u64,
    msg_out: u64,
    bytes_in: u64,
    bytes_out: u64,
}

struct State {}

struct Options {
    send_timeo: Duration,
    recv_timeo: Duration,
    reconn_timeo: Duration,

}

pub struct Path(String);

pub struct Channel {
    state: State,
    stats: Stats,
    options: Options,
    time: Times,
}

unsafe impl Sync for Channel {}
