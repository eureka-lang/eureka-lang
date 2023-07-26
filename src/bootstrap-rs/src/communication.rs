use crate::eureka::Token;

pub use position::Position;
mod position;

pub const INVALID_VALUE: &str = "invalid value";

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct PositionError {
    pub position: Position,
    pub error: Error,
}

impl PositionError {
    pub fn new(position: Position, error: Error) -> PositionError {
        PositionError { position, error }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Error {
    Expected(&'static str),
    ExpectedToken(Token),
    UnexpectedChar(char),
    UnexpectedEndOfFile,
    UnexpectedToken(Token),
}
