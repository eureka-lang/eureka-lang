use keyword::Keyword;

mod keyword;
mod name;

enum Token {
    Keyword(Keyword),
}
