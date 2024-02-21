use crate::wasm::Encode;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Instruction {}

impl Encode for Instruction {
    fn encode(&self, _buffer: &mut Vec<u8>) {}
}
