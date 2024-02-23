pub use encode::Encode;
mod encode;

pub use expression::Expression;
mod expression;

pub use instruction::Instruction;
mod instruction;

pub use instruction_sequence::InstructionSequence;
mod instruction_sequence;

mod leb128;

pub use module::Module;
mod module;

mod types;

pub use vector::Vector;
mod vector;
