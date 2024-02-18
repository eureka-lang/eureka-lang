use crate::wasm::types::NumberType;
use crate::wasm::Encode;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ValueType {
    NumberType(NumberType),
}

impl Encode for ValueType {
    fn encode(&self, buffer: &mut Vec<u8>) {
        match self {
            Self::NumberType(number_type) => number_type.encode(buffer),
        }
    }
}

impl From<NumberType> for ValueType {
    fn from(value: NumberType) -> ValueType {
        ValueType::NumberType(value)
    }
}
