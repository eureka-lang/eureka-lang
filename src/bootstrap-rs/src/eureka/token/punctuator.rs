use crate::eureka::code::Code;
use crate::eureka::token::Token;
use crate::miscellaneous::DisplayName;
use std::fmt;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Punctuator {
    LeftParenthesis,
    RightParenthesis,
    LeftBrace,
    RightBrace,
}

impl Punctuator {
    pub fn lex(src: &str) -> Option<(Punctuator, &str)> {
        let mut code = Code::new(src);
        match Self::lex2(&mut code) {
            None => None,
            Some(punctuator) => Some((punctuator, &src[punctuator.unlex().len()..])),
        }
    }

    pub fn lex2(code: &mut Code) -> Option<Self> {
        let punctuator = match code.peek() {
            Some('(') => Self::LeftParenthesis,
            Some(')') => Self::RightParenthesis,
            Some('{') => Self::LeftBrace,
            Some('}') => Self::RightBrace,
            _ => return None,
        };

        code.pop();

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
        for (src, expected_punctuator, expected_remaining_src) in [
            ("(a", Punctuator::LeftParenthesis, "a"),
            (")", Punctuator::RightParenthesis, ""),
            ("{\n    ", Punctuator::LeftBrace, "\n    "),
            ("} else {\r\n", Punctuator::RightBrace, " else {\r\n"),
        ] {
            let (actual_punctuator, actual_remaining_src) = Punctuator::lex(src).unwrap();

            assert_eq!(expected_punctuator, actual_punctuator);
            assert_eq!(expected_remaining_src, actual_remaining_src);
        }
    }

    #[test]
    fn lex_fails() {
        for src in ["", "x", "1", "if", " ", "#"] {
            assert!(Punctuator::lex(src).is_none());
        }
    }
}
