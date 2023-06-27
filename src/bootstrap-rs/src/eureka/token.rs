use crate::text::Position;
pub use identifier::Identifier;
pub use keyword::Keyword;
pub use padding::Padding;
pub use punctuator::Punctuator;
pub use tokens::Tokens;

mod identifier;
mod keyword;
mod lex;
mod name;
mod padding;
mod punctuator;
mod tokens;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Token {
    Identifier(Identifier),
    Keyword(Keyword),
    Padding(Padding),
    Punctuator(Punctuator),
}

impl Token {
    pub fn lex(src: &str) -> Option<(Token, &str)> {
        if let Some((identifier, remaining_src)) = Identifier::lex(src) {
            return Some((identifier.into(), remaining_src));
        }

        if let Some((keyword, remaining_src)) = Keyword::lex(src) {
            return Some((keyword.into(), remaining_src));
        }

        if let Some((padding, remaining_src)) = Padding::lex(src) {
            return Some((padding.into(), remaining_src));
        }

        if let Some((punctuator, remaining_src)) = Punctuator::lex(src) {
            return Some((punctuator.into(), remaining_src));
        }

        None
    }

    pub fn lex_all(mut src: &str) -> Result<Vec<Token>, Position> {
        let mut result = Vec::new();
        let mut current_position = Position::new(1, 1);

        while !src.is_empty() {
            if let Some((token, remaining_src)) = Token::lex(src) {
                current_position.relative_move(token.relative_end());
                result.push(token);
                src = remaining_src;
            } else {
                return Err(current_position);
            }
        }

        Ok(result)
    }

    pub fn relative_end(&self) -> Position {
        match self {
            Self::Identifier(identifier) => identifier.relative_end(),
            Self::Keyword(keyword) => keyword.relative_end(),
            Self::Padding(padding) => padding.relative_end(),
            Self::Punctuator(punctuator) => punctuator.relative_end(),
        }
    }
}

impl From<Identifier> for Token {
    fn from(value: Identifier) -> Token {
        Token::Identifier(value)
    }
}

impl From<Keyword> for Token {
    fn from(value: Keyword) -> Token {
        Token::Keyword(value)
    }
}

impl From<Padding> for Token {
    fn from(value: Padding) -> Token {
        Token::Padding(value)
    }
}

impl From<Punctuator> for Token {
    fn from(value: Punctuator) -> Token {
        Token::Punctuator(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_empty_main() {
        let src = "fn main() {}";

        let (token, src) = Token::lex(src).unwrap();
        assert_eq!(token, Keyword::Fn.into());

        let (token, src) = Token::lex(src).unwrap();
        assert_eq!(token, Padding::new(" ").into());

        let (token, src) = Token::lex(src).unwrap();
        assert_eq!(token, Identifier::new("main").into());

        let (token, src) = Token::lex(src).unwrap();
        assert_eq!(token, Punctuator::LeftParenthesis.into());

        let (token, src) = Token::lex(src).unwrap();
        assert_eq!(token, Punctuator::RightParenthesis.into());

        let (token, src) = Token::lex(src).unwrap();
        assert_eq!(token, Padding::new(" ").into());

        let (token, src) = Token::lex(src).unwrap();
        assert_eq!(token, Punctuator::LeftBrace.into());

        let (token, src) = Token::lex(src).unwrap();
        assert_eq!(token, Punctuator::RightBrace.into());

        assert!(src.is_empty());
    }

    #[test]
    fn lex_all_empty_main() {
        let src = "fn main() {}";
        let actual_tokens = Token::lex_all(src).unwrap();
        let expected_tokens: Vec<Token> = vec![
            Keyword::Fn.into(),
            Padding::new(" ").into(),
            Identifier::new("main").into(),
            Punctuator::LeftParenthesis.into(),
            Punctuator::RightParenthesis.into(),
            Padding::new(" ").into(),
            Punctuator::LeftBrace.into(),
            Punctuator::RightBrace.into(),
        ];

        assert_eq!(expected_tokens, actual_tokens);
    }

    #[test]
    fn lex_all_empty_string() {
        let tokens = Token::lex_all("").unwrap();
        assert!(tokens.is_empty());
    }

    #[test]
    fn lex_fails() {
        for src in ["", "\0", "\x1B"] {
            assert!(Token::lex(src).is_none());
        }
    }

    #[test]
    fn lex_all_fails() {
        assert_eq!(Token::lex_all("\x1B"), Err(Position::new(1, 1)));

        assert_eq!(Token::lex_all("\0"), Err(Position::new(1, 1)));
        assert_eq!(Token::lex_all("fn\0"), Err(Position::new(1, 3)));
        assert_eq!(Token::lex_all("fn \0"), Err(Position::new(1, 4)));
        assert_eq!(Token::lex_all("fn main\0"), Err(Position::new(1, 8)));
        assert_eq!(Token::lex_all("fn main(\0"), Err(Position::new(1, 9)));
        assert_eq!(Token::lex_all("fn main()\0"), Err(Position::new(1, 10)));
        assert_eq!(Token::lex_all("fn main()\n\0"), Err(Position::new(2, 1)));
        assert_eq!(Token::lex_all("fn main()\n{\0"), Err(Position::new(2, 2)));
        assert_eq!(Token::lex_all("fn main()\n{\n\0"), Err(Position::new(3, 1)));
        assert_eq!(
            Token::lex_all("fn main()\n{\n}\0"),
            Err(Position::new(3, 2)),
        );
    }
}
