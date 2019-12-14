use crate::channel;
use uuid;
use bytes::{Bytes, BufMut, Buf, BytesMut};

pub struct Id(uuid::Uuid);

impl Id {
    pub fn new() -> Self {
        Id(uuid::v4::new())
    }
}

pub trait Message {
    fn id(&self) -> &Id;
    fn index(&self) -> usize;
    fn len(&self) -> usize;
    fn payload(&self) -> &[u8];
}

pub struct Msg {
    offset: usize,
    index: usize,
}

pub struct Queue {
    bytes: BytesMut,
    offset: usize,
    first_idx: usize,
}

impl Iterator for Queue {
    type Item = Msg;

    fn next(&mut self) -> Option<Self::Item> {

    }
}

impl Queue {
    pub fn new(offset: usize, cap: usize) -> Self {
        let bytes = BytesMut::with_capacity(cap);
        Self {
            bytes,
            offset = 
        }
    }
}


impl Msg {
    pub fn new(chan: channel::Path, data: Data) -> Self {
        Msg {}
    }
}
