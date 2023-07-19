use crate::communication::Error;
use crate::eureka::chars::Chars;
pub use identifier::Identifier;
pub use keyword::Keyword;
pub use padding::Padding;
pub use punctuator::Punctuator;
use std::fmt;

mod identifier;
mod keyword;
mod padding;
mod punctuator;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Token {
    Identifier(Identifier),
    Keyword(Keyword),
    Padding(Padding),
    Punctuator(Punctuator),
}

impl Token {
    pub fn lex(chars: &mut Chars) -> Result<Option<Token>, Error> {
        if let Some(token) = Identifier::lex(chars) {
            match token {
                Ok(identifier) => return Ok(Some(Token::Identifier(identifier))),
                Err(keyword) => return Ok(Some(Token::Keyword(keyword))),
            }
        }

        if let Some(padding) = Padding::lex(chars)? {
            return Ok(Some(Token::Padding(padding)));
        }

        if let Some(punctuator) = Punctuator::lex(chars) {
            return Ok(Some(Token::Punctuator(punctuator)));
        }

        match chars.peek() {
            None => Ok(None),
            Some(c) => Err(Error::UnexpectedChar(c)),
        }
    }

    pub fn unlex(&self) -> &str {
        match self {
            Self::Identifier(identifier) => identifier.unlex(),
            Self::Keyword(keyword) => keyword.unlex(),
            Self::Padding(padding) => padding.unlex(),
            Self::Punctuator(punctuator) => punctuator.unlex(),
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Identifier(identifier) => fmt::Display::fmt(identifier, f),
            Self::Keyword(keyword) => fmt::Display::fmt(keyword, f),
            Self::Padding(padding) => fmt::Display::fmt(padding, f),
            Self::Punctuator(punctuator) => fmt::Display::fmt(punctuator, f),
        }
    }
}

impl From<Identifier> for Token {
    fn from(value: Identifier) -> Token {
        Token::Identifier(value)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_empty_main() {
        let mut chars = Chars::new("fn main() {}");

        let token = Token::lex(&mut chars).unwrap().unwrap();
        assert_eq!(token, Keyword::Fn.into());

        let token = Token::lex(&mut chars).unwrap().unwrap();
        assert_eq!(token, Padding::new(" ").into());

        let token = Token::lex(&mut chars).unwrap().unwrap();
        assert_eq!(token, Identifier::new("main").into());

        let token = Token::lex(&mut chars).unwrap().unwrap();
        assert_eq!(token, Punctuator::LeftParenthesis.into());

        let token = Token::lex(&mut chars).unwrap().unwrap();
        assert_eq!(token, Punctuator::RightParenthesis.into());

        let token = Token::lex(&mut chars).unwrap().unwrap();
        assert_eq!(token, Padding::new(" ").into());

        let token = Token::lex(&mut chars).unwrap().unwrap();
        assert_eq!(token, Punctuator::LeftBrace.into());

        let token = Token::lex(&mut chars).unwrap().unwrap();
        assert_eq!(token, Punctuator::RightBrace.into());

        assert!(Token::lex(&mut chars).unwrap().is_none());
        assert!(chars.peek().is_none());
    }

    #[test]
    fn lex_empty() {
        let mut chars = Chars::new("");

        assert!(chars.peek().is_none());
        assert!(Token::lex(&mut chars).unwrap().is_none());

        assert!(chars.peek().is_none());
        assert!(Token::lex(&mut chars).unwrap().is_none());

        assert!(chars.peek().is_none());
    }
}
