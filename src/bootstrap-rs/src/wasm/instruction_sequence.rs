use crate::wasm::types::{FunctionType, ResultType};
use crate::wasm::{Encode, Instruction};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct InstructionSequence {
    function_type: FunctionType,
    instructions: Vec<Instruction>,
}

impl InstructionSequence {
    pub const fn new() -> InstructionSequence {
        InstructionSequence {
            function_type: FunctionType::new(),
            instructions: Vec::new(),
        }
    }

    pub fn input_types(&self) -> &ResultType {
        &self.function_type.parameter_types
    }

    pub fn output_types(&self) -> &ResultType {
        &self.function_type.result_types
    }
}

impl Encode for InstructionSequence {
    fn encode(&self, buffer: &mut Vec<u8>) {
        for instruction in self.instructions.iter() {
            instruction.encode(buffer);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_empty() {
        let instruction_sequence = InstructionSequence::new();

        let mut buffer = Vec::new();
        instruction_sequence.encode(&mut buffer);

        assert!(buffer.is_empty());
    }
}
