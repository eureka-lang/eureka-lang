use super::super::keyword::Keyword;
use super::super::name::lex_unquoted_name;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct UnquotedIdentifier<'a> {
    value: &'a str,
}

impl<'a> UnquotedIdentifier<'a> {
    pub fn as_str(&self) -> &str {
        self.value
    }

    pub fn lex(src: &str) -> Option<(UnquotedIdentifier, &str)> {
        if Keyword::lex(src).is_some() {
            return None;
        }

        if let Some((name, remaining_src)) = lex_unquoted_name(src) {
            let identifier = UnquotedIdentifier { value: name };
            return Some((identifier, remaining_src));
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
