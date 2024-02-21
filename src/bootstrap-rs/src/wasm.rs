pub use encode::Encode;
mod encode;

pub use instruction::Instruction;
mod instruction;

mod leb128;
mod types;

pub use vector::Vector;
mod vector;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Module;

impl Module {
    const MAGIC_NUMBER: [u8; 4] = [0x00, 0x61, 0x73, 0x6D];
    const VERSION: [u8; 4] = [0x01, 0x00, 0x00, 0x00];

    pub fn new() -> Module {
        Module
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();

        buffer.extend(Self::MAGIC_NUMBER);
        buffer.extend(Self::VERSION);

        buffer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_empty() {
        let module = Module::new();

        let actual = module.encode();
        let expected = vec![
            0x00, 0x61, 0x73, 0x6D, // magic number
            0x01, 0x00, 0x00, 0x00, // version
        ];

        assert_eq!(expected, actual);
    }
}
