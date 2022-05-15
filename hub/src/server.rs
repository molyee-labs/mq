use crate::channel;
use crate::cluster::Cluster;
use crate::config;
use crate::store;
use crate::node;
use crate::net::{Connection, UdpHub};
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;
use timer::{self, SyncTimer};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

type TrieMap<T> = HashMap<String, T>; // TODO

struct Version(&'static str);

#[derive(Default)]
struct Statistics {
    msg_in: u64,
    msg_out: u64,
    bytes_in: u64,
    bytes_out: u64,
    //slow_receivers: u64,
}

impl Statistics {
    fn new() -> Self {
        Default::default()
    }
}

pub struct State {
    stats: Statistics,
    channels: TrieMap<channel::Channel>,
}

impl State {
    fn new() -> Self {
        let stats = Statistics::new();
        let channels = HashMap::new();
        State {
            stats,
            channels,
        }
    }
}

pub struct Info {
    id: node::Id,
    version: Version,
    git_commit: &'static str,
}

impl Info {
    pub fn new(id: node::Id) -> Self {
        Info {
            id,
            version: Version(VERSION),
            git_commit: "unknown",
        }
    }
}

pub struct Server {
    info: Info,
    state: State,
    cluster: Cluster,
    storage: store::Storage,
    timer: timer::SyncTimer,
}

impl Server {
    pub fn new(conf: &config::Config) -> Self {
        let node_id = node::Id::new();
        let info = Info::new(node_id);
        let timer = timer::run::<SyncTimer>();
        let state = State::new();
        let storage = store::Storage::new(&conf.storage);
        let cluster = Cluster::new(&conf.cluster);
        let hub = UdpHub::bind(conf.network, &cluster);
        Self {
            info,
            state,
            cluster,
            storage,
            timer
        }
    }

    pub fn run(&self) {
        unimplemented!()
    }

    pub fn stop(&self) {
        unimplemented!()
    }
}
