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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Token<'a> {
    Identifier(Identifier<'a>),
    Keyword(Keyword),
    Padding(Padding<'a>),
    Punctuator(Punctuator),
}

impl<'a> From<Identifier<'a>> for Token<'a> {
    fn from(value: Identifier) -> Token {
        Token::Identifier(value)
    }
}

impl<'a> From<UnquotedIdentifier<'a>> for Token<'a> {
    fn from(value: UnquotedIdentifier) -> Token {
        Token::Identifier(value.into())
    }
}

impl<'a> From<Keyword> for Token<'a> {
    fn from(value: Keyword) -> Token<'a> {
        Token::Keyword(value)
    }
}

impl<'a> From<Padding<'a>> for Token<'a> {
    fn from(value: Padding) -> Token {
        Token::Padding(value)
    }
}

impl<'a> From<Punctuator> for Token<'a> {
    fn from(value: Punctuator) -> Token<'a> {
        Token::Punctuator(value)
    }
}
