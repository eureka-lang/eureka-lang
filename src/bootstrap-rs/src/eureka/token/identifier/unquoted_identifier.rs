use super::super::lex;
pub use restricted::UnquotedIdentifier;

mod restricted {
    use super::super::super::keyword::Keyword;
    use super::super::super::name::lex_unquoted_name;

    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub struct UnquotedIdentifier {
        value: String,
    }

    impl UnquotedIdentifier {
        pub fn as_str(&self) -> &str {
            self.value.as_str()
        }

        pub fn lex(src: &str) -> Option<(UnquotedIdentifier, &str)> {
            if Keyword::lex(src).is_some() {
                return None;
            }

            if let Some((name, remaining_src)) = lex_unquoted_name(src) {
                let identifier = UnquotedIdentifier {
                    value: String::from(name),
                };

                return Some((identifier, remaining_src));
            }

            None
        }
    }
}

impl UnquotedIdentifier {
    pub fn new(value: &str) -> UnquotedIdentifier {
        lex::entirely(UnquotedIdentifier::lex)(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_succeeds() {
        let identifier = UnquotedIdentifier::new("i");
        assert_eq!("i", identifier.as_str());
    }

    #[test]
    #[should_panic(expected = "invalid value")]
    fn new_fails_if_value_is_keyword() {
        let _ = UnquotedIdentifier::new("if");
    }

    #[test]
    #[should_panic(expected = "invalid value")]
    fn new_fails_if_value_is_not_entirely_identifier() {
        let _ = UnquotedIdentifier::new("i+");
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
            let (actual_identifier, actual_remaining_src) = UnquotedIdentifier::lex(src).unwrap();

            assert_eq!(expected_identifier, actual_identifier.as_str());
            assert_eq!(expected_remaining_src, actual_remaining_src);
        }
    }

    #[test]
    fn lex_fails() {
        for src in ["", "99", "if", "return", "#if", "+"] {
            assert!(UnquotedIdentifier::lex(src).is_none());
        }
    }
}
