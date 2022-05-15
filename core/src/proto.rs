pub struct Id(u8);

pub trait Protocol {
    fn id(&self) -> Id;
    fn name(&self) -> &str;
    fn send(&self, data: &[u8]) -> Result<usize>;
    fn recv(&self) -> Result<usize>;
}
