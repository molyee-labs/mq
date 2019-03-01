use std::net::*;
use std::collections::HashMap;
use timer::{self, SyncTimer};
use crate::config;
use crate::channel;
use crate::client;
use crate::cluster;
use crate::store;

struct Statistics {

}

pub struct State {
    stats: Statistics,
    clients: HashMap<client::Id, client::Client>,
    channels: channel::Map,
}

pub struct Server {
    state: State,
    cluster: cluster::Cluster,
    config: config::Config,
    storage: store::Storage,
    timer: timer::SyncTimer,
    listener: TcpListener,
}

impl Server {
    pub fn start(conf: &config::Config) -> Self {
        let timer = timer::run::<SyncTimer>();
        let listener = TcpListener::bind(&conf.node.addr).unwrap();
        unimplemented!();
    }

    pub fn stop(&self) {
        unimplemented!();
    }
}
