use crate::eureka::token::Token;
pub use position::Position;

mod position;

pub const INVALID_VALUE: &str = "invalid value";

pub trait DisplayName {
    fn display_name() -> &'static str;
}

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
    Missing(&'static str),
    MissingToken(Token),
    UnexpectedChar(char),
    UnexpectedCharOrEndOfFile(Option<char>),
    UnexpectedToken(Token),
}
