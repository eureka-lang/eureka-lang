use crate::collections::Push;
use crate::wasm::types::NumberType;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ValueType {
    NumberType(NumberType),
}

impl ValueType {
    pub fn encode(&self, buffer: &mut impl Push<u8>) {
        match self {
            Self::NumberType(number_type) => number_type.encode(buffer),
        }
    }
}
