use crate::wasm::{leb128, Encode};
pub use restricted::Vector;
use std::ops::Index;

mod restricted {
    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub struct Vector<T> {
        values: Vec<T>,
    }

    impl<T> Vector<T> {
        const MAX_LEN: usize = u32::MAX as usize;

        pub const fn new() -> Vector<T> {
            Vector { values: Vec::new() }
        }

        pub const fn values(&self) -> &Vec<T> {
            &self.values
        }

        pub fn push(&mut self, value: T) {
            if self.values.len() >= Self::MAX_LEN {
                panic!("vector max len exceeded");
            }

            self.values.push(value);
        }
    }
}

impl<T> Vector<T> {
    pub fn len(&self) -> u32 {
        self.values().len() as u32
    }
}

impl<T> Index<u32> for Vector<T> {
    type Output = T;

    fn index(&self, index: u32) -> &Self::Output {
        let index_usize: usize = index.try_into().unwrap();
        &self.values()[index_usize]
    }
}

impl<T: Encode> Encode for Vector<T> {
    fn encode(&self, buffer: &mut Vec<u8>) {
        leb128::encode_u32(self.len(), buffer);

        for i in 0..self.len() {
            self[i].encode(buffer);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wasm::types::NumberType;

    #[test]
    fn test_vector() {
        let mut vector = Vector::new();
        assert_eq!(0, vector.len());

        vector.push(NumberType::I32);
        assert_eq!(1, vector.len());
        assert_eq!(NumberType::I32, vector[0]);

        vector.push(NumberType::I64);
        assert_eq!(2, vector.len());
        assert_eq!(NumberType::I32, vector[0]);
        assert_eq!(NumberType::I64, vector[1]);
    }

    #[test]
    fn encode_empty() {
        let vector: Vector<NumberType> = Vector::new();

        let mut actual_buffer = Vec::new();
        vector.encode(&mut actual_buffer);

        let expected_buffer = vec![
            0x00, // vector length
        ];

        assert_eq!(expected_buffer, actual_buffer);
    }

    #[test]
    fn encode_non_empty() {
        let mut vector = Vector::new();

        vector.push(NumberType::I32);
        vector.push(NumberType::I32);
        vector.push(NumberType::I64);

        let mut actual_buffer = Vec::new();
        vector.encode(&mut actual_buffer);

        let expected_buffer = vec![
            0x03, // vector length
            0x7F, // type i32
            0x7F, // type i32
            0x7E, // type i64
        ];

        assert_eq!(expected_buffer, actual_buffer);
    }
}
