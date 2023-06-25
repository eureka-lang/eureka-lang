use super::unquoted_identifier::UnquotedIdentifier;
use crate::text::Position;

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

    pub fn relative_end(&self) -> Position {
        match self {
            Self::Unquoted(identifier) => identifier.relative_end(),
        }
    }
}

impl From<UnquotedIdentifier> for Identifier {
    fn from(value: UnquotedIdentifier) -> Identifier {
        Identifier::Unquoted(value)
    }
}
