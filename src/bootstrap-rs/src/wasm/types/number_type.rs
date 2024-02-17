use crate::wasm::Encode;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum NumberType {
    I32,
    I64,
}

impl Encode for NumberType {
    fn encode(&self, buffer: &mut Vec<u8>) {
        let byte = match self {
            Self::I32 => 0x7F,
            Self::I64 => 0x7E,
        };

        buffer.push(byte);
    }
}
