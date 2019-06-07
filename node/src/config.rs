use url::HostAndPort;
use std::path::Path;
use std::time::Duration;
use merge_derive::Mix;

pub struct NodeConf {
    pub addr: HostAndPort,
}

pub struct ClusterConf {
    pub listen: HostAndPort,
    pub routes: Vec<HostAndPort>,
    pub timeout: Duration,
}

pub enum StoreTarget {
    File(Box<Path>),
}

pub struct StorageConf {
    pub name: String,
    pub target: StoreTarget,
}

#[derive(Mix)]
pub struct AuthConf {
    
}

pub struct Config {
    pub node: NodeConf,
    pub cluster: Option<ClusterConf>,
    pub storage: Option<StorageConf>,
    pub auth: Option<AuthConf>,
}
