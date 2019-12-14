use std::path::Path;
use std::time::Duration;
use url::HostAndPort;
use crate::node;
use crate::protocol;
use crate::transport;

#[derive(Debug)]
pub enum Error {
    BadSocketAddr(HostAndPort),
}

pub struct ClusterConf {
    pub node_id: node::Id,
}

pub struct HubConf {
    pub proto: transport::Protocol,
    pub peers: Vec<HostAndPort>,
    pub listen: HostAndPort,
    pub timeout: Duration,
}

pub enum StoreTarget {
    File(Box<Path>),
}

pub struct StorageConf {
    pub name: String,
    pub target: StoreTarget,
}

pub struct AuthConf {}

pub struct Config {
    pub cluster: ClusterConf,
    pub network: NetConf,
    pub storage: StorageConf,
    pub auth: Option<AuthConf>,
}

pub fn load(file: &str) -> Config {
    unimplemented!()
}
