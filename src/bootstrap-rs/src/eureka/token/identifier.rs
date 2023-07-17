use super::lex;
use crate::communication::DisplayName;
use crate::eureka::token::Token;
pub use restricted::Identifier;
use std::fmt;

mod restricted {
    use super::super::keyword::Keyword;
    use crate::eureka::chars::Chars;

    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub struct Identifier {
        value: String,
    }

    impl Identifier {
        pub fn lex(src: &str) -> Option<(Identifier, &str)> {
            let mut chars = Chars::new(src);

            if let Some(Ok(identifier)) = Self::lex2(&mut chars) {
                let len = identifier.unlex().len();
                Some((identifier, &src[len..]))
            } else {
                None
            }
        }

        pub fn lex2(chars: &mut Chars) -> Option<Result<Identifier, Keyword>> {
            if let Some('a'..='z' | 'A'..='Z' | '_') = chars.peek() {
                let mut value = String::new();
                value.push(chars.pop().unwrap());

                while let Some('a'..='z' | 'A'..='Z' | '_' | '0'..='9') = chars.peek() {
                    value.push(chars.pop().unwrap());
                }

                if let Some(keyword) = Keyword::lex2(&value) {
                    return Some(Err(keyword));
                } else {
                    return Some(Ok(Identifier { value }));
                }
            }

            None
        }

        pub fn unlex(&self) -> &str {
            self.value.as_str()
        }
    }
}

impl Identifier {
    pub fn new(value: &str) -> Identifier {
        lex::entirely(Identifier::lex)(value)
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", self.unlex())
    }
}

impl DisplayName for Identifier {
    fn display_name() -> &'static str {
        "identifier"
    }
}

impl TryFrom<Token> for Identifier {
    type Error = ();

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::Identifier(identifier) => Ok(identifier),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_succeeds() {
        let identifier = Identifier::new("i");
        assert_eq!("i", identifier.unlex());
    }

    #[test]
    #[should_panic(expected = "invalid value")]
    fn new_fails_if_value_is_keyword() {
        let _ = Identifier::new("if");
    }

    #[test]
    #[should_panic(expected = "invalid value")]
    fn new_fails_if_value_is_not_entirely_identifier() {
        let _ = Identifier::new("i+");
    }

    #[test]
    fn lex_succeeds() {
        for (src, expected_identifier, expected_remaining_src) in [
            ("_", "_", ""),
            ("c", "c", ""),
            ("i += 1;", "i", " += 1;"),
            ("if_;", "if_", ";"),
            ("a_z__A_Z__0_9()", "a_z__A_Z__0_9", "()"),
        ] {
            let (actual_identifier, actual_remaining_src) = Identifier::lex(src).unwrap();

            assert_eq!(expected_identifier, actual_identifier.unlex());
            assert_eq!(expected_remaining_src, actual_remaining_src);
        }
    }

    #[test]
    fn lex_fails() {
        for src in ["", "99", "if", "return", "#if", "+"] {
            assert!(Identifier::lex(src).is_none());
        }
    }
}
