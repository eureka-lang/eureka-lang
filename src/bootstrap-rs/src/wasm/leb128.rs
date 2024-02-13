use crate::collections::Push;

pub fn encode_u64(mut value: u64, buffer: &mut impl Push<u8>) {
    loop {
        let least_significant_byte = value as u8;
        value >>= 7;

        if value != 0 {
            buffer.push(least_significant_byte | 0x80);
        } else {
            buffer.push(least_significant_byte);
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_u64() {
        for (value, expected_buffer) in [
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
            let mut actual_buffer = Vec::new();
            encode_u64(value, &mut actual_buffer);
            assert_eq!(expected_buffer, actual_buffer);
        }
    }
}
