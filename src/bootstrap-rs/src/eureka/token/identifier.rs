use super::lex;
use crate::eureka::token::Token;
use crate::miscellaneous::DisplayName;
pub use restricted::Identifier;
use std::fmt;

mod restricted {
    use super::super::keyword::Keyword;
    use super::super::name::lex_unquoted_name;

    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub struct Identifier {
        value: String,
    }

    impl Identifier {
        pub fn lex(src: &str) -> Option<(Identifier, &str)> {
            if Keyword::lex(src).is_some() {
                return None;
            }

            if let Some((name, remaining_src)) = lex_unquoted_name(src) {
                let identifier = Identifier {
                    value: String::from(name),
                };

                return Some((identifier, remaining_src));
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
