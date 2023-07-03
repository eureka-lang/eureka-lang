use super::Token;
use crate::text::Position;

pub struct Lexer {
    values: Vec<Token>,
    position: Position,
}

impl Lexer {
    pub fn new(mut values: Vec<Token>) -> Lexer {
        values.reverse();
        Lexer {
            values,
            position: Position::new(1, 1),
        }
    }

    pub fn lex_all(src: &str) -> Result<Lexer, Position> {
        Token::lex_all(src).map(Lexer::new)
    }

    pub fn peek(&self) -> Option<&Token> {
        self.values.last()
    }

    pub fn pop(&mut self) -> Option<Token> {
        match self.values.pop() {
            None => None,
            Some(value) => {
                self.position.relative_move(value.relative_end());
                Some(value)
            }
        }
    }

    pub fn position(&self) -> Position {
        self.position
    }
}

#[cfg(test)]
mod tests {
    use super::super::{Identifier, Keyword, Padding};
    use super::*;

    #[test]
    fn empty_lexer() {
        let mut lexer = Lexer::new(Vec::new());

        assert_eq!(lexer.peek(), None);
        assert_eq!(lexer.pop(), None);
        assert_eq!(lexer.position(), Position::new(1, 1));
    }

    #[test]
    fn non_empty_lexer() {
        let mut lexer = Lexer::lex_all("fn example").unwrap();

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
