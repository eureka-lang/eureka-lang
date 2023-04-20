use identifier::Identifier;
use keyword::Keyword;
use padding::Padding;
use punctuator::Punctuator;

mod identifier;
mod keyword;
mod name;
mod padding;
mod punctuator;

enum Token<'a> {
    Identifier(Identifier<'a>),
    Keyword(Keyword),
    Padding(Padding<'a>),
    Punctuator(Punctuator),
}
