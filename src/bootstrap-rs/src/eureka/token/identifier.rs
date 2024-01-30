use crate::communication::{Error, INVALID_VALUE};
use crate::eureka::{Chars, Token};
pub use restricted::Identifier;

mod restricted {
    use crate::eureka::{Chars, Keyword};

    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub struct Identifier {
        value: String,
    }

    impl Identifier {
        pub fn lex(chars: &mut Chars) -> Option<Result<Identifier, Keyword>> {
            let mut value = String::new();

            if chars.try_take(|c| matches!(c, 'a'..='z' | 'A'..='Z' | '_'), &mut value) {
                chars.take_while(
                    |c| matches!(c, 'a'..='z' | 'A'..='Z' | '_' | '0'..='9'),
                    &mut value,
                );

                match Keyword::lex(&value) {
                    Some(keyword) => Some(Err(keyword)),
                    None => Some(Ok(Identifier { value })),
                }
            } else {
                None
            }
        }

        pub fn unlex(&self) -> &str {
            self.value.as_str()
        }
    }
}

impl Identifier {
    pub fn new(value: &str) -> Identifier {
        let mut chars = Chars::new(value);

        if let Some(Ok(identifier)) = Self::lex(&mut chars) {
            if chars.peek().is_none() {
                return identifier;
            }
        }

        panic!("{INVALID_VALUE}");
    }
}

impl TryFrom<Option<Token>> for Identifier {
    type Error = Error;

    fn try_from(value: Option<Token>) -> Result<Self, Self::Error> {
        match value {
            Some(Token::Identifier(identifier)) => Ok(identifier),
            _ => Err(Error::Expected("identifier")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eureka::Keyword;

    #[test]
    fn new() {
        let identifier = Identifier::new("i");
        assert_eq!("i", identifier.unlex());
    }

    #[test]
    #[should_panic(expected = "invalid value")]
    fn new_panics_if_value_is_keyword() {
        let _ = Identifier::new("if");
    }

    #[test]
    #[should_panic(expected = "invalid value")]
    fn new_panics_if_value_is_not_entirely_identifier() {
        let _ = Identifier::new("i+");
    }

    #[test]
    fn lex_some_identifier() {
        for (src, expected_identifier, expected_peek) in [
            ("_", "_", None),
            ("c", "c", None),
            ("a+b", "a", Some('+')),
            ("T: ", "T", Some(':')),
            ("i += 1;", "i", Some(' ')),
            ("if_;", "if_", Some(';')),
            ("fnX", "fnX", None),
            ("fn2", "fn2", None),
            ("a_z__A_Z__0_9()", "a_z__A_Z__0_9", Some('(')),
        ] {
            let mut chars = Chars::new(src);
            let actual_identifier = Identifier::lex(&mut chars).unwrap().unwrap();

            assert_eq!(expected_identifier, actual_identifier.unlex());
            assert_eq!(expected_peek, chars.peek());
        }
    }

    #[test]
    fn lex_some_keyword() {
        for (src, expected_keyword, expected_peek) in [
            ("fn", Keyword::Fn, None),
            ("if", Keyword::If, None),
            ("return", Keyword::Return, None),
            ("fn main", Keyword::Fn, Some(' ')),
            ("if (a < b) {}", Keyword::If, Some(' ')),
            ("return (0);", Keyword::Return, Some(' ')),
        ] {
            let mut chars = Chars::new(src);
            let actual_keyword = Identifier::lex(&mut chars).unwrap().unwrap_err();

            assert_eq!(expected_keyword, actual_keyword);
            assert_eq!(expected_peek, chars.peek());
        }
    }

    #[test]
    fn lex_none() {
        for src in [
            "", " ", "1", "2x", "99", "#if", "$", "+", "-", "@", "[", "^", "`", "{",
        ] {
            let mut chars = Chars::new(src);
            assert!(Identifier::lex(&mut chars).is_none());
            assert_eq!(src.chars().next(), chars.peek());
        }
    }
}
