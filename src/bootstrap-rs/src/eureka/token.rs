use identifier::Identifier;
use keyword::Keyword;
use padding::Padding;

mod identifier;
mod keyword;
mod name;
mod padding;

enum Token<'a> {
    Identifier(Identifier<'a>),
    Keyword(Keyword),
    Padding(Padding<'a>),
}
