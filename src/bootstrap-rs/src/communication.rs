use crate::eureka::token::Token;
pub use position::Position;

mod position;

pub trait DisplayName {
    fn display_name() -> &'static str;
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct PositionError {
    pub position: Position,
    pub error: Error,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Error {
    Missing(&'static str),
    MissingToken(Token),
    UnexpectedCharOrEndOfFile(Option<char>),
    UnexpectedToken(Token),
}
