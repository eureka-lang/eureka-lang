use crate::communication::DisplayName;
use crate::eureka::chars::Chars;
use crate::eureka::token::Token;
use std::fmt;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Punctuator {
    LeftParenthesis,
    RightParenthesis,
    LeftBrace,
    RightBrace,
}

impl Punctuator {
    pub fn lex(chars: &mut Chars) -> Option<Punctuator> {
        let punctuator = match chars.peek() {
            Some('(') => Self::LeftParenthesis,
            Some(')') => Self::RightParenthesis,
            Some('{') => Self::LeftBrace,
            Some('}') => Self::RightBrace,
            _ => return None,
        };

        chars.pop();

        Some(punctuator)
    }

    pub fn unlex(&self) -> &'static str {
        match self {
            Self::LeftParenthesis => "(",
            Self::RightParenthesis => ")",
            Self::LeftBrace => "{",
            Self::RightBrace => "}",
        }
    }
}

impl fmt::Display for Punctuator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", self.unlex())
    }
}

impl DisplayName for Punctuator {
    fn display_name() -> &'static str {
        "punctuator"
    }
}

impl TryFrom<Token> for Punctuator {
    type Error = ();

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::Punctuator(punctuator) => Ok(punctuator),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_succeeds() {
        for (src, expected_punctuator, expected_peek) in [
            ("(a", Punctuator::LeftParenthesis, Some('a')),
            (")", Punctuator::RightParenthesis, None),
            ("{\n    ", Punctuator::LeftBrace, Some('\n')),
            ("} else", Punctuator::RightBrace, Some(' ')),
        ] {
            let mut chars = Chars::new(src);
            let actual_punctuator = Punctuator::lex(&mut chars).unwrap();

            assert_eq!(expected_punctuator, actual_punctuator);
            assert_eq!(expected_peek, chars.peek());
        }
    }

    #[test]
    fn lex_fails() {
        for src in ["", "x", "1", "if", " ", "#"] {
            let mut chars = Chars::new(src);
            assert!(Punctuator::lex(&mut chars).is_none());
            assert_eq!(src.chars().next(), chars.peek());
        }
    }
}
