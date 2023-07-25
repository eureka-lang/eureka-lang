use crate::communication::Error;
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

    pub fn try_take<T: TryFrom<Option<Token>, Error = Error>>(&mut self) -> Option<T> {
        self.take().ok()
    }

    pub fn take<T: TryFrom<Option<Token>, Error = Error>>(&mut self) -> Result<T, Error> {
        let result = T::try_from(self.peek());

        if result.is_ok() {
            self.pop();
        }

        result
    }

    pub fn expect<T: Into<Token>>(&mut self, token: T) -> Result<(), Error> {
        let expected_token: Token = token.into();

        if let Some(actual_token) = self.peek() {
            if actual_token == expected_token {
                self.pop();
                return Ok(());
            }
        }

        Err(Error::MissingToken(expected_token))
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
    fn try_take() {
        let mut tokens = Tokens::new(" fn");

        assert_eq!(tokens.peek(), Some(Token::Padding(Padding::new(" "))));

        let padding1 = tokens.try_take::<Padding>();

        assert_eq!(padding1, Some(Padding::new(" ")));
        assert_eq!(tokens.peek(), Some(Token::Keyword(Keyword::Fn)));

        let padding2 = tokens.try_take::<Padding>();

        assert_eq!(padding2, None);
        assert_eq!(tokens.peek(), Some(Token::Keyword(Keyword::Fn)));
    }

    #[test]
    fn take() {
        let mut tokens = Tokens::new(" fn");

        assert_eq!(tokens.peek(), Some(Token::Padding(Padding::new(" "))));

        let padding1 = tokens.take::<Padding>();

        assert_eq!(padding1, Ok(Padding::new(" ")));
        assert_eq!(tokens.peek(), Some(Token::Keyword(Keyword::Fn)));

        let padding2 = tokens.take::<Padding>();

        assert_eq!(padding2, Err(Error::Expected("padding")));
        assert_eq!(tokens.peek(), Some(Token::Keyword(Keyword::Fn)));
    }

    #[test]
    fn expect() {
        let mut tokens = Tokens::new("fn(value");
        assert_eq!(tokens.peek(), Some(Keyword::Fn.into()));

        assert_eq!(
            Err(Error::MissingToken(Keyword::Return.into())),
            tokens.expect(Keyword::Return),
        );
        assert_eq!(tokens.peek(), Some(Keyword::Fn.into()));

        assert_eq!(
            Err(Error::MissingToken(Identifier::new("value").into())),
            tokens.expect(Identifier::new("value")),
        );
        assert_eq!(tokens.peek(), Some(Keyword::Fn.into()));

        assert_eq!(Ok(()), tokens.expect(Keyword::Fn));
        assert_eq!(tokens.peek(), Some(Punctuator::LeftParenthesis.into()));

        assert_eq!(Ok(()), tokens.expect(Punctuator::LeftParenthesis));
        assert_eq!(tokens.peek(), Some(Identifier::new("value").into()));

        assert_eq!(Ok(()), tokens.expect(Identifier::new("value")));
        assert_eq!(tokens.peek(), None);

        assert_eq!(
            Err(Error::MissingToken(Punctuator::RightParenthesis.into())),
            tokens.expect(Punctuator::RightParenthesis),
        );
        assert_eq!(tokens.peek(), None);
    }
}
