use crate::config::ClusterConf;
use crate::node::{self, Node};
use crate::net::{Hub};
use std::cell::RefCell;
use std::collections::HashMap;

pub struct Cluster<H: Hub> {
    current: node::Id,
    master: RefCell<node::Id>,
    hub: H,
    nodes: HashMap<node::Id, Node>,
}

impl<H: Hub> Cluster<H> {
    pub fn new(hub: H, conf: &ClusterConf) -> Self {
        let current = conf.node_id;
        let master = RefCell::new(current);
        let nodes = HashMap::new();
        Self {
            current,
            master,
            hub,
            nodes,
        }
    }

    fn add(&self, id: node::Id, node: Node) {
        self.nodes.insert(id, node);
    }

    fn remove(&self, id: &node::Id) -> Option<Node> {
        self.nodes.remove(id)
    }

    fn get(&self, id: &node::Id) -> Option<&Node> {
        self.nodes.get(id)
    }

    fn local(&self) -> &Node {
        self.get(&self.current).unwrap()
    }
}
