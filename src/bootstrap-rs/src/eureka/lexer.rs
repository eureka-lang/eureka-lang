use crate::communication::{Position, PositionError};
use crate::eureka::token::Token;

pub struct Lexer {
    tokens: Vec<Token>,
    position: Position,
}

impl Lexer {
    pub fn new(mut tokens: Vec<Token>) -> Lexer {
        tokens.reverse();
        Lexer {
            tokens,
            position: Position::start(),
        }
    }

    pub fn try_new(src: &str) -> Result<Lexer, PositionError> {
        Token::lex_all(src).map(Lexer::new)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eureka::token::{Identifier, Keyword, Padding};

    #[test]
    fn empty_lexer() {
        let mut lexer = Lexer::new(Vec::new());

        assert_eq!(lexer.peek(), None);
        assert_eq!(lexer.pop(), None);
        assert_eq!(lexer.position(), Position::start());
    }

    #[test]
    fn non_empty_lexer() {
        let mut lexer = Lexer::try_new("fn example").unwrap();

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
}
