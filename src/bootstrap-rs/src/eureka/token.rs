use crate::communication::Error;
use crate::eureka::Chars;
pub use identifier::Identifier;
pub use keyword::Keyword;
pub use padding::Padding;
pub use punctuation::Punctuation;

mod identifier;
mod keyword;
mod padding;
mod punctuation;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Token {
    Identifier(Identifier),
    Keyword(Keyword),
    Padding(Padding),
    Punctuation(Punctuation),
}

impl Token {
    pub fn lex(chars: &mut Chars) -> Result<Option<Token>, Error> {
        if let Some(token) = Identifier::lex(chars) {
            match token {
                Ok(identifier) => return Ok(Some(Self::Identifier(identifier))),
                Err(keyword) => return Ok(Some(Self::Keyword(keyword))),
            }
        }

        if let Some(padding) = Padding::lex(chars)? {
            return Ok(Some(Self::Padding(padding)));
        }

        if let Some(punctuation) = Punctuation::lex(chars) {
            return Ok(Some(Self::Punctuation(punctuation)));
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
            Self::Punctuation(punctuation) => punctuation.unlex(),
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

impl From<Punctuation> for Token {
    fn from(value: Punctuation) -> Token {
        Token::Punctuation(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_empty() {
        let mut chars = Chars::new("");

        assert!(chars.peek().is_none());
        assert!(Token::lex(&mut chars).unwrap().is_none());

        assert!(chars.peek().is_none());
        assert!(Token::lex(&mut chars).unwrap().is_none());

        assert!(chars.peek().is_none());
    }

    #[test]
    fn lex_non_empty() {
        let mut chars = Chars::new("fn main() {}");

        let token = Token::lex(&mut chars).unwrap().unwrap();
        assert_eq!(token, Keyword::Fn.into());

        let token = Token::lex(&mut chars).unwrap().unwrap();
        assert_eq!(token, Padding::new(" ").into());

        let token = Token::lex(&mut chars).unwrap().unwrap();
        assert_eq!(token, Identifier::new("main").into());

        let token = Token::lex(&mut chars).unwrap().unwrap();
        assert_eq!(token, Punctuation::LeftParenthesis.into());

        let token = Token::lex(&mut chars).unwrap().unwrap();
        assert_eq!(token, Punctuation::RightParenthesis.into());

        let token = Token::lex(&mut chars).unwrap().unwrap();
        assert_eq!(token, Padding::new(" ").into());

        let token = Token::lex(&mut chars).unwrap().unwrap();
        assert_eq!(token, Punctuation::LeftBrace.into());

        let token = Token::lex(&mut chars).unwrap().unwrap();
        assert_eq!(token, Punctuation::RightBrace.into());

        assert!(Token::lex(&mut chars).unwrap().is_none());
        assert!(chars.peek().is_none());
    }
}
