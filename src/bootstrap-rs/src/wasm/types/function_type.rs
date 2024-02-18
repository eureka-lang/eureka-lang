use crate::wasm::types::ResultType;
use crate::wasm::Encode;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct FunctionType {
    pub parameter_types: ResultType,
    pub result_types: ResultType,
}

impl FunctionType {
    pub const fn new() -> FunctionType {
        FunctionType {
            parameter_types: ResultType::new(),
            result_types: ResultType::new(),
        }
    }
}

impl Encode for FunctionType {
    fn encode(&self, buffer: &mut Vec<u8>) {
        buffer.push(0x60);

        self.parameter_types.encode(buffer);
        self.result_types.encode(buffer);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wasm::types::NumberType;

    #[test]
    fn encode_empty() {
        let function_type = FunctionType::new();

        let mut actual_buffer = Vec::new();
        function_type.encode(&mut actual_buffer);

        let expected_buffer: Vec<u8> = vec![
            0x60, // function type
            0x00, // parameter types length
            0x00, // result types length
        ];

        assert_eq!(expected_buffer, actual_buffer);
    }

    #[test]
    fn encode_non_empty() {
        let mut function_type = FunctionType::new();

        function_type.parameter_types.push(NumberType::I32.into());
        function_type.parameter_types.push(NumberType::I32.into());
        function_type.result_types.push(NumberType::I64.into());

        let mut actual_buffer = Vec::new();
        function_type.encode(&mut actual_buffer);

        let expected_buffer: Vec<u8> = vec![
            0x60, // function type
            0x02, // parameter types length
            0x7F, // type i32
            0x7F, // type i32
            0x01, // result types length
            0x7E, // type i64
        ];

        assert_eq!(expected_buffer, actual_buffer);
    }
}
