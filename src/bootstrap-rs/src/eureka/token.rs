use identifier::unquoted_identifier::UnquotedIdentifier;
use identifier::Identifier;
use keyword::Keyword;
use padding::Padding;
use punctuator::Punctuator;

mod identifier;
mod keyword;
mod name;
mod padding;
mod punctuator;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Token {
    Identifier(Identifier),
    Keyword(Keyword),
    Padding(Padding),
    Punctuator(Punctuator),
}

impl From<Identifier> for Token {
    fn from(value: Identifier) -> Token {
        Token::Identifier(value)
    }
}

impl From<UnquotedIdentifier> for Token {
    fn from(value: UnquotedIdentifier) -> Token {
        Token::Identifier(value.into())
    }
}

impl From<Keyword> for Token {
    fn from(value: Keyword) -> Token {
        Token::Keyword(value)
    }
}

impl From<Padding> for Token {
    fn from(value: Padding) -> Token {
        Token::Padding(value)
    }
}

impl From<Punctuator> for Token {
    fn from(value: Punctuator) -> Token {
        Token::Punctuator(value)
    }
}
