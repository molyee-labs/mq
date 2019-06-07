use std::net::*;
use std::collections::HashMap;
use std::cell::RefCell;
use timer::{self, SyncTimer};
use crate::config;
use crate::channel;
use crate::client;
use crate::cluster;
use crate::store;
use tree::TrieMap;

type Id = uuid::Uuid;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

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
    clients: HashMap<client::Id, client::Client>,
    channels: TrieMap<channel::Channel>,
}

impl State {
    fn new() -> Self {
        let stats = Statistics::new();
        let clients = HashMap::new();
        let channels = TrieMap::new();
        State { stats, clients, channels }
    }
}

pub struct Info {
    id: Id,
    version: Version, 
    git_commit: &'static str,
}

pub struct Server {
    state: RefCell<State>,
    cluster: cluster::Cluster,
    config: config::Config,
    info: Info,
    storage: store::Storage,
    timer: timer::SyncTimer,
    listener: TcpListener,
}

impl Server {
    pub fn start(conf: &config::Config) -> Self {
        let state = RefCell::from(State::new());
        let timer = timer::run::<SyncTimer>();
        let listener = TcpListener::bind(&conf.node.addr).unwrap();
        unimplemented!()
    }

    pub fn stop(&self) {
        unimplemented!()
    }
}
