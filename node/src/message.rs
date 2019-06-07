use crate::channel;
use uuid::Uuid;

pub struct Id(Uuid);

impl Id {
    pub fn new() -> Self {
        Id(uuid::v4::new())
    }
}

pub struct Data {
    raw: Box<[u8]>,
}

pub struct Msg {
    id: Id,
    chan: channel::Path,
    data: Data,
}

impl Msg {
    pub fn new(chan: channel::Path, data: Data) -> Self {
        let id = Id::new();
        Msg { id, chan, data }
    }
}
