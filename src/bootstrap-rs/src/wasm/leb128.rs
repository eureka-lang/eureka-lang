pub struct EncodeU64 {
    value: Option<u64>,
}

pub fn encode_u64(value: u64) -> EncodeU64 {
    EncodeU64 { value: Some(value) }
}

impl Iterator for EncodeU64 {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        match self.value {
            None => None,
            Some(value) => {
                let least_significant_byte = value as u8;
                let new_value = value >> 7;

                if new_value != 0 {
                    self.value = Some(new_value);
                    Some(least_significant_byte | 0x80)
                } else {
                    self.value = None;
                    Some(least_significant_byte)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_u64() {
        for (value, expected) in [
            (0, vec![0x00]),
            (1, vec![0x01]),
            (126, vec![0x7E]),
            (127, vec![0x7F]),
            (128, vec![0x80, 0x01]),
            (129, vec![0x81, 0x01]),
            (16382, vec![0xFE, 0x7F]),
            (16383, vec![0xFF, 0x7F]),
            (16384, vec![0x80, 0x80, 0x01]),
            (16385, vec![0x81, 0x80, 0x01]),
            (
                58985701577553908,
                vec![0xF4, 0xEF, 0xED, 0xEA, 0xED, 0xE5, 0xE3, 0x68],
            ),
            (
                u64::MAX,
                vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01],
            ),
        ] {
            let actual = Vec::from_iter(encode_u64(value));
            assert_eq!(expected, actual);
        }
    }
}
