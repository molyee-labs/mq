pub type Id = Uuid;

pub trait Message {
    fn id(&self) -> Id;
    fn payload(&self) -> &[u8];
    fn as_bytes(&self) -> &[u8];
}