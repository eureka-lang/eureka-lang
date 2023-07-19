pub use restricted::Lexer;

mod restricted {
    use crate::communication::{Position, PositionError};
    use crate::eureka::chars::Chars;
    use crate::eureka::token::Token;

    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub struct Lexer {
        tokens: Vec<Token>,
        position: Position,
    }

    impl Lexer {
        pub fn try_new(src: &str) -> Result<Lexer, PositionError> {
            let mut chars = Chars::try_new(src)?;
            let mut tokens = Vec::new();
            let mut position = Position::start();

            loop {
                match Token::lex(&mut chars) {
                    Ok(Some(token)) => {
                        position.advance_str(token.unlex());
                        tokens.push(token);
                    }
                    Ok(None) => break,
                    Err(e) => return Err(PositionError::new(position, e)),
                }
            }

            tokens.reverse();

            Ok(Lexer {
                tokens,
                position: Position::start(),
            })
        }

        pub fn peek(&self) -> Option<&Token> {
            self.tokens.last()
        }

        pub fn pop(&mut self) -> Option<Token> {
            match self.tokens.pop() {
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

impl Lexer {
    pub fn new(src: &str) -> Lexer {
        Lexer::try_new(src).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::communication::Position;
    use crate::eureka::token::{Identifier, Keyword, Padding, Token};

    #[test]
    fn empty() {
        let mut lexer = Lexer::new("");

        assert_eq!(lexer, Lexer::try_new("").unwrap());

        assert_eq!(lexer.peek(), None);
        assert_eq!(lexer.pop(), None);
        assert_eq!(lexer.position(), Position::start());
    }

    #[test]
    fn non_empty() {
        let mut lexer = Lexer::new("fn example");

        assert_eq!(lexer.position(), Position::new(1, 1));
        assert_eq!(lexer.peek(), Some(&Token::from(Keyword::Fn)));
        assert_eq!(lexer.pop(), Some(Token::from(Keyword::Fn)));

        assert_eq!(lexer.position(), Position::new(1, 3));
        assert_eq!(lexer.peek(), Some(&Token::from(Padding::new(" "))));
        assert_eq!(lexer.pop(), Some(Token::from(Padding::new(" "))));

        assert_eq!(lexer.position(), Position::new(1, 4));
        assert_eq!(lexer.peek(), Some(&Token::from(Identifier::new("example"))));
        assert_eq!(lexer.pop(), Some(Token::from(Identifier::new("example"))));

        assert_eq!(lexer.position(), Position::new(1, 11));
        assert_eq!(lexer.peek(), None);
        assert_eq!(lexer.pop(), None);

        assert_eq!(lexer.position(), Position::new(1, 11));
        assert_eq!(lexer.peek(), None);
        assert_eq!(lexer.pop(), None);
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
            let position_error = Lexer::try_new(src).unwrap_err();
            assert_eq!(position_error.position, expected_position);
        }
    }
}
