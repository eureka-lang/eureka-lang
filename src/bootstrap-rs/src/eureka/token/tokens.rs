use super::Token;
use crate::text::Position;

pub struct Tokens {
    values: Vec<Token>,
    position: Position,
}

impl Tokens {
    pub fn new(mut values: Vec<Token>) -> Tokens {
        values.reverse();
        Tokens {
            values,
            position: Position::new(1, 1),
        }
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
    use super::super::{Keyword, Padding, UnquotedIdentifier};
    use super::*;

    #[test]
    fn empty_tokens() {
        let mut tokens = Tokens::new(Vec::new());

        assert_eq!(tokens.peek(), None);
        assert_eq!(tokens.pop(), None);
        assert_eq!(tokens.position(), Position::new(1, 1));
    }

    #[test]
    fn tokens() {
        let tokens = Token::lex_all("fn example").unwrap();
        let mut tokens = Tokens::new(tokens);

        assert_eq!(tokens.position(), Position::new(1, 1));
        assert_eq!(tokens.peek(), Some(&Token::from(Keyword::Fn)));
        assert_eq!(tokens.pop(), Some(Token::from(Keyword::Fn)));

        assert_eq!(tokens.position(), Position::new(1, 3));
        assert_eq!(tokens.peek(), Some(&Token::from(Padding::new(" "))));
        assert_eq!(tokens.pop(), Some(Token::from(Padding::new(" "))));

        assert_eq!(tokens.position(), Position::new(1, 4));
        assert_eq!(
            tokens.peek(),
            Some(&Token::from(UnquotedIdentifier::new("example"))),
        );
        assert_eq!(
            tokens.pop(),
            Some(Token::from(UnquotedIdentifier::new("example"))),
        );

        assert_eq!(tokens.position(), Position::new(1, 11));
        assert_eq!(tokens.peek(), None);
        assert_eq!(tokens.pop(), None);

        assert_eq!(tokens.position(), Position::new(1, 11));
        assert_eq!(tokens.peek(), None);
        assert_eq!(tokens.pop(), None);
    }
}
