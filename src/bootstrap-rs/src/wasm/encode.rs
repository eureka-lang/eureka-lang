pub trait Encode {
    fn encode(&self, buffer: &mut Vec<u8>);
}
