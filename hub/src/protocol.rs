use crate::node;

#[derive(Copy, Clone)]
pub enum Variant {
    Udp,
    Tcp
}

pub trait Protocol {
    fn sender(&self) -> node::Id;
    fn variant(&self) -> Variant;
}
