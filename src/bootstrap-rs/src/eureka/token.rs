use crate::communication::{Error, Position};
use crate::eureka::chars::Chars;
pub use identifier::Identifier;
pub use keyword::Keyword;
pub use padding::Padding;
pub use punctuator::Punctuator;
use std::fmt;

mod identifier;
mod keyword;
mod lex;
mod name;
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
    pub fn lex(src: &str) -> Option<(Token, &str)> {
        let mut chars = Chars::new(src);

        if let Ok(Some(token)) = Self::lex2(&mut chars) {
            let len = token.unlex().len();
            Some((token, &src[len..]))
        } else {
            None
        }
    }

    pub fn lex2(chars: &mut Chars) -> Result<Option<Token>, Error> {
        if let Some(token) = Identifier::lex2(chars) {
            match token {
                Ok(identifier) => return Ok(Some(Token::Identifier(identifier))),
                Err(keyword) => return Ok(Some(Token::Keyword(keyword))),
            }
        }

        if let Some(padding) = Padding::lex2(chars)? {
            return Ok(Some(Token::Padding(padding)));
        }

        if let Some(punctuator) = Punctuator::lex2(chars) {
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

    pub fn lex_all(mut src: &str) -> Result<Vec<Token>, Position> {
        let mut result = Vec::new();
        let mut current_position = Position::start();

        while !src.is_empty() {
            if let Some((token, remaining_src)) = Token::lex(src) {
                current_position.advance_str(token.unlex());
                result.push(token);
                src = remaining_src;
            } else {
                return Err(current_position);
            }
        }

        Ok(result)
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
        let src = "fn main() {}";

        let (token, src) = Token::lex(src).unwrap();
        assert_eq!(token, Keyword::Fn.into());

        let (token, src) = Token::lex(src).unwrap();
        assert_eq!(token, Padding::new(" ").into());

        let (token, src) = Token::lex(src).unwrap();
        assert_eq!(token, Identifier::new("main").into());

        let (token, src) = Token::lex(src).unwrap();
        assert_eq!(token, Punctuator::LeftParenthesis.into());

        let (token, src) = Token::lex(src).unwrap();
        assert_eq!(token, Punctuator::RightParenthesis.into());

        let (token, src) = Token::lex(src).unwrap();
        assert_eq!(token, Padding::new(" ").into());

        let (token, src) = Token::lex(src).unwrap();
        assert_eq!(token, Punctuator::LeftBrace.into());

        let (token, src) = Token::lex(src).unwrap();
        assert_eq!(token, Punctuator::RightBrace.into());

        assert!(src.is_empty());
    }

    #[test]
    fn lex_all_empty_main() {
        let src = "fn main() {}";
        let actual_tokens = Token::lex_all(src).unwrap();
        let expected_tokens: Vec<Token> = vec![
            Keyword::Fn.into(),
            Padding::new(" ").into(),
            Identifier::new("main").into(),
            Punctuator::LeftParenthesis.into(),
            Punctuator::RightParenthesis.into(),
            Padding::new(" ").into(),
            Punctuator::LeftBrace.into(),
            Punctuator::RightBrace.into(),
        ];

        assert_eq!(expected_tokens, actual_tokens);
    }

    #[test]
    fn lex_all_empty_string() {
        let tokens = Token::lex_all("").unwrap();
        assert!(tokens.is_empty());
    }

    #[test]
    fn lex_fails() {
        assert!(Token::lex("").is_none());
    }

    #[test]
    fn lex_all_fails() {
        assert_eq!(Token::lex_all("`"), Err(Position::new(1, 1)));
        assert_eq!(Token::lex_all("fn`"), Err(Position::new(1, 3)));
        assert_eq!(Token::lex_all("fn `"), Err(Position::new(1, 4)));
        assert_eq!(Token::lex_all("fn main`"), Err(Position::new(1, 8)));
        assert_eq!(Token::lex_all("fn main(`"), Err(Position::new(1, 9)));
        assert_eq!(Token::lex_all("fn main()`"), Err(Position::new(1, 10)));
        assert_eq!(Token::lex_all("fn main()\n`"), Err(Position::new(2, 1)));
        assert_eq!(Token::lex_all("fn main()\n{`"), Err(Position::new(2, 2)));
        assert_eq!(Token::lex_all("fn main()\n{\n`"), Err(Position::new(3, 1)));
        assert_eq!(Token::lex_all("fn main()\n{\n}`"), Err(Position::new(3, 2)));
    }
}
