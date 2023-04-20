use unquoted_identifier::UnquotedIdentifier;

mod unquoted_identifier;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Identifier<'a> {
    Unquoted(UnquotedIdentifier<'a>),
}

impl<'a> Identifier<'a> {
    pub fn lex(src: &str) -> Option<(Identifier, &str)> {
        if let Some((identifier, remaining_src)) = UnquotedIdentifier::lex(src) {
            return Some((Identifier::Unquoted(identifier), remaining_src));
        }

        None
    }
}
