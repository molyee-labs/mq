use std::cell::{RefCell};
use std::net::TcpListener;
use crate::config::ClusterConf;
use crate::net::TcpConnection;

pub enum Node {
    Local(TcpListener),
    Remote(TcpConnection),
}

pub struct Cluster {
    current: Node,
    master: RefCell<isize>,
    nodes: Vec<Node>,
}

impl Cluster {
    pub fn new(conf: &ClusterConf) -> Self {
        let mut nodes: Vec<Node> = Vec::with_capacity(conf.routes.len());
        let listener = TcpListener::bind(&conf.listen).unwrap();
        let current = Node::Local(listener);
        for addr in conf.routes.iter() {
            let conn = TcpConnection::connect(addr, conf.timeout);
            nodes.push(Node::Remote(conn));
        }
        let master = RefCell::new(-1);
        Cluster { current, master, nodes }
    }
}

