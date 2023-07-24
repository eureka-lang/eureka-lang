use crate::communication::{DisplayName, Error};
use crate::eureka::token::Token;
pub use restricted::Tokens;

mod restricted {
    use crate::communication::{Position, PositionError};
    use crate::eureka::chars::Chars;
    use crate::eureka::token::Token;

    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub struct Tokens {
        values: Vec<Token>,
        position: Position,
    }

    impl Tokens {
        pub fn try_new(src: &str) -> Result<Tokens, PositionError> {
            let mut chars = Chars::try_new(src)?;
            let mut values = Vec::new();

            loop {
                match Token::lex(&mut chars) {
                    Ok(Some(token)) => values.push(token),
                    Ok(None) => break,
                    Err(e) => return Err(PositionError::new(chars.position(), e)),
                }
            }

            values.reverse();

            Ok(Tokens {
                values,
                position: Position::start(),
            })
        }

        pub fn peek(&self) -> Option<Token> {
            self.values.last().cloned()
        }

        pub fn pop(&mut self) -> Option<Token> {
            match self.values.pop() {
                None => None,
                Some(token) => {
                    self.position.advance_str(token.unlex());
                    Some(token)
                }
            }
        }

        pub fn position(&self) -> Position {
            self.position
        }
    }
}

impl Tokens {
    pub fn new(src: &str) -> Tokens {
        Tokens::try_new(src).unwrap()
    }

    pub fn optional<T: TryFrom<Token>>(&mut self) -> Option<T> {
        if let Some(token) = self.peek() {
            if let Ok(t) = T::try_from(token) {
                self.pop();
                return Some(t);
            }
        }

        None
    }

    pub fn required<T: DisplayName + TryFrom<Token>>(&mut self) -> Result<T, Error> {
        self.optional()
            .ok_or_else(|| Error::Missing(T::display_name()))
    }

    pub fn expected<T>(&mut self, expected_token: T) -> Result<(), Error>
    where
        T: Eq + PartialEq + Into<Token> + TryFrom<Token>,
    {
        if let Some(token) = self.peek() {
            if let Ok(actual_token) = T::try_from(token) {
                if expected_token == actual_token {
                    self.pop();
                    return Ok(());
                }
            }
        }

        Err(Error::MissingToken(expected_token.into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::communication::Position;
    use crate::eureka::token::{Identifier, Keyword, Padding, Punctuator, Token};

    #[test]
    fn empty() {
        let mut tokens = Tokens::new("");

        assert_eq!(tokens, Tokens::try_new("").unwrap());

        assert_eq!(tokens.peek(), None);
        assert_eq!(tokens.pop(), None);
        assert_eq!(tokens.position(), Position::start());
    }

    #[test]
    fn non_empty() {
        let mut tokens = Tokens::new("fn example");

        assert_eq!(tokens.position(), Position::new(1, 1));
        assert_eq!(tokens.peek(), Some(Token::from(Keyword::Fn)));
        assert_eq!(tokens.pop(), Some(Token::from(Keyword::Fn)));

        assert_eq!(tokens.position(), Position::new(1, 3));
        assert_eq!(tokens.peek(), Some(Token::from(Padding::new(" "))));
        assert_eq!(tokens.pop(), Some(Token::from(Padding::new(" "))));

        assert_eq!(tokens.position(), Position::new(1, 4));
        assert_eq!(tokens.peek(), Some(Token::from(Identifier::new("example"))));
        assert_eq!(tokens.pop(), Some(Token::from(Identifier::new("example"))));

        assert_eq!(tokens.position(), Position::new(1, 11));
        assert_eq!(tokens.peek(), None);
        assert_eq!(tokens.pop(), None);

        assert_eq!(tokens.position(), Position::new(1, 11));
        assert_eq!(tokens.peek(), None);
        assert_eq!(tokens.pop(), None);
    }

    #[test]
    fn non_empty_error() {
        for (src, expected_position) in [
            ("`", Position::new(1, 1)),
            ("fn`", Position::new(1, 3)),
            ("fn `", Position::new(1, 4)),
            ("fn main`", Position::new(1, 8)),
            ("fn main(`", Position::new(1, 9)),
            ("fn main()`", Position::new(1, 10)),
            ("fn main()\n`", Position::new(2, 1)),
            ("fn main()\n{`", Position::new(2, 2)),
            ("fn main()\n{\n`", Position::new(3, 1)),
            ("fn main()\n{\n}`", Position::new(3, 2)),
        ] {
            let position_error = Tokens::try_new(src).unwrap_err();
            assert_eq!(position_error.position, expected_position);
        }
    }

    #[test]
    fn optional() {
        let mut tokens = Tokens::new(" fn");

        assert_eq!(tokens.peek(), Some(Token::Padding(Padding::new(" "))));

        let padding1 = tokens.optional::<Padding>();

        assert_eq!(padding1, Some(Padding::new(" ")));
        assert_eq!(tokens.peek(), Some(Token::Keyword(Keyword::Fn)));

        let padding2 = tokens.optional::<Padding>();

        assert_eq!(padding2, None);
        assert_eq!(tokens.peek(), Some(Token::Keyword(Keyword::Fn)));
    }

    #[test]
    fn required() {
        let mut tokens = Tokens::new(" fn");

        assert_eq!(tokens.peek(), Some(Token::Padding(Padding::new(" "))));

        let padding1 = tokens.required::<Padding>();

        assert_eq!(padding1, Ok(Padding::new(" ")));
        assert_eq!(tokens.peek(), Some(Token::Keyword(Keyword::Fn)));

        let padding2 = tokens.required::<Padding>();

        assert_eq!(padding2, Err(Error::Missing("padding")));
        assert_eq!(tokens.peek(), Some(Token::Keyword(Keyword::Fn)));
    }

    #[test]
    fn expected() {
        let mut tokens = Tokens::new("fn(value");
        assert_eq!(tokens.peek(), Some(Keyword::Fn.into()));

        assert_eq!(
            Err(Error::MissingToken(Keyword::Return.into())),
            tokens.expected(Keyword::Return),
        );
        assert_eq!(tokens.peek(), Some(Keyword::Fn.into()));

        assert_eq!(
            Err(Error::MissingToken(Identifier::new("value").into())),
            tokens.expected(Identifier::new("value")),
        );
        assert_eq!(tokens.peek(), Some(Keyword::Fn.into()));

        assert_eq!(Ok(()), tokens.expected(Keyword::Fn));
        assert_eq!(tokens.peek(), Some(Punctuator::LeftParenthesis.into()));

        assert_eq!(Ok(()), tokens.expected(Punctuator::LeftParenthesis));
        assert_eq!(tokens.peek(), Some(Identifier::new("value").into()));

        assert_eq!(Ok(()), tokens.expected(Identifier::new("value")));
        assert_eq!(tokens.peek(), None);

        assert_eq!(
            Err(Error::MissingToken(Punctuator::RightParenthesis.into())),
            tokens.expected(Punctuator::RightParenthesis),
        );
        assert_eq!(tokens.peek(), None);
    }
}
