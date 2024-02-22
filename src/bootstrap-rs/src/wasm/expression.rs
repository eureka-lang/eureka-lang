use crate::wasm::{Encode, InstructionSequence};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Expression {
    instruction_sequence: InstructionSequence,
}

impl Expression {
    pub fn try_new(instruction_sequence: InstructionSequence) -> Option<Expression> {
        if instruction_sequence.input_types().is_empty() {
            Some(Expression {
                instruction_sequence,
            })
        } else {
            None
        }
    }

    pub fn instruction_sequence(&self) -> &InstructionSequence {
        &self.instruction_sequence
    }
}

impl Encode for Expression {
    fn encode(&self, buffer: &mut Vec<u8>) {
        self.instruction_sequence.encode(buffer);
        buffer.push(0x0B);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_empty() {
        let instruction_sequence = InstructionSequence::new();
        let expression = Expression::try_new(instruction_sequence).unwrap();

        let mut actual_buffer = Vec::new();
        expression.encode(&mut actual_buffer);

        let expected_buffer = vec![
            0x0B, // end expression
        ];

        assert_eq!(expected_buffer, actual_buffer);
    }
}
