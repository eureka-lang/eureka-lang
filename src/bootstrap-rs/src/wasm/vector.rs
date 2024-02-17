use std::ops::Index;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Vector<T> {
    values: Vec<T>,
}

impl<T> Vector<T> {
    const MAX_LEN: usize = u32::MAX as usize;

    pub const fn new() -> Vector<T> {
        Vector { values: Vec::new() }
    }

    pub fn push(&mut self, value: T) {
        if self.values.len() >= Self::MAX_LEN {
            panic!("vector max len exceeded");
        }

        self.values.push(value);
    }

    pub fn len(&self) -> u32 {
        self.values.len() as u32
    }
}

impl<T> Index<u32> for Vector<T> {
    type Output = T;

    fn index(&self, index: u32) -> &Self::Output {
        let index_usize: usize = index.try_into().unwrap();
        &self.values[index_usize]
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
}
