use unquoted_identifier::UnquotedIdentifier;

pub mod unquoted_identifier;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Identifier {
    Unquoted(UnquotedIdentifier),
}

impl Identifier {
    pub fn lex(src: &str) -> Option<(Identifier, &str)> {
        if let Some((identifier, remaining_src)) = UnquotedIdentifier::lex(src) {
            return Some((Identifier::Unquoted(identifier), remaining_src));
        }

        None
    }
}

impl From<UnquotedIdentifier> for Identifier {
    fn from(value: UnquotedIdentifier) -> Identifier {
        Identifier::Unquoted(value)
    }
}
