use crate::client;
use std::time;
use std::vec::Vec;
use std::sync::Arc;

struct Times {
    created: time::Instant,
    last_msg: time::Duration,
}

struct Subscription {
    client: client::Id,
    times: time::Duration, 
}

struct Statistics {

}

struct State {

}

enum Space {
    Group(String),
    Any,
    All,
}

impl From<&str> for Space {
    fn from(from: &str) -> Self {
        match from {
            "*" => Space::Any,
            "+" => Space::All,
            g => Space::Group(g.to_string()),
        }
    }
}

struct Stream {
    state: State,
    stats: Statistics,
    time: Times,
    subscriptions: Vec<Subscription>,
}

pub struct Channel {
    space: Space,
    stream: Stream,
    children: Vec<Map>,
}

pub struct Path(Vec<Space>);

impl From<&str> for Path {
    fn from(from: &str) -> Self {
        let spaces = from.split(".");
        let spaces = spaces.into_iter().map(|s| Space::from(s)).collect();
        Path(spaces)
    }
}

pub struct Map {
    root: Arc<Channel>
}