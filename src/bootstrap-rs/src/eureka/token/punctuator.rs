use crate::eureka::token::Token;
use crate::miscellaneous::DisplayName;
use crate::text::Position;
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
        let mut chars = src.chars();

        let punctuator = match chars.next() {
            Some('(') => Punctuator::LeftParenthesis,
            Some(')') => Punctuator::RightParenthesis,
            Some('{') => Punctuator::LeftBrace,
            Some('}') => Punctuator::RightBrace,
            _ => return None,
        };

        Some((punctuator, chars.as_str()))
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::LeftParenthesis => "(",
            Self::RightParenthesis => ")",
            Self::LeftBrace => "{",
            Self::RightBrace => "}",
        }
    }

    pub fn len(&self) -> usize {
        self.as_str().len()
    }

    pub fn relative_end(&self) -> Position {
        Position::new(1, self.len() + 1)
    }
}

impl fmt::Display for Punctuator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
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

    #[test]
    fn relative_end() {
        assert_eq!(
            Punctuator::LeftParenthesis.relative_end(),
            Position::new(1, 2),
        );
        assert_eq!(
            Punctuator::RightParenthesis.relative_end(),
            Position::new(1, 2),
        );
    }
}
