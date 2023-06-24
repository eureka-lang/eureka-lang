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

    pub fn len(&self) -> usize {
        match self {
            Self::LeftParenthesis => 1,
            Self::RightParenthesis => 1,
            Self::LeftBrace => 1,
            Self::RightBrace => 1,
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
