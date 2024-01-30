use crate::eureka::Chars;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Punctuation {
    LeftParenthesis,
    RightParenthesis,
    LeftBrace,
    RightBrace,
}

impl Punctuation {
    pub fn lex(chars: &mut Chars) -> Option<Punctuation> {
        let punctuation = match chars.peek() {
            Some('(') => Self::LeftParenthesis,
            Some(')') => Self::RightParenthesis,
            Some('{') => Self::LeftBrace,
            Some('}') => Self::RightBrace,
            _ => return None,
        };

        chars.pop();

        Some(punctuation)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_some() {
        for (src, expected_punctuation, expected_peek) in [
            ("(a", Punctuation::LeftParenthesis, Some('a')),
            (")", Punctuation::RightParenthesis, None),
            ("{\n    ", Punctuation::LeftBrace, Some('\n')),
            ("} else", Punctuation::RightBrace, Some(' ')),
        ] {
            let mut chars = Chars::new(src);
            let actual_punctuation = Punctuation::lex(&mut chars).unwrap();

            assert_eq!(expected_punctuation, actual_punctuation);
            assert_eq!(expected_peek, chars.peek());
        }
    }

    #[test]
    fn lex_none() {
        for src in ["", "x", "1", "if", " ", "#"] {
            let mut chars = Chars::new(src);
            assert!(Punctuation::lex(&mut chars).is_none());
            assert_eq!(src.chars().next(), chars.peek());
        }
    }
}
