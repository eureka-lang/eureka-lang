use identifier::Identifier;
use keyword::Keyword;

mod identifier;
mod keyword;
mod name;

enum Token<'a> {
    Identifier(Identifier<'a>),
    Keyword(Keyword),
}
