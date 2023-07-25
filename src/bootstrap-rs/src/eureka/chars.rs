use crate::communication::Error;
pub use restricted::Chars;

mod restricted {
    use crate::communication::Error::UnexpectedChar;
    use crate::communication::{Position, PositionError};

    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub struct Chars {
        values: Vec<char>,
        position: Position,
    }

    impl Chars {
        pub fn try_new(src: &str) -> Result<Chars, PositionError> {
            let mut values: Vec<char> = Vec::with_capacity(src.len());
            let mut position = Position::start();

            for c in src.chars() {
                if (' ' <= c && c <= '~') || c == '\n' {
                    values.push(c);
                    position.advance(c);
                } else {
                    return Err(PositionError::new(position, UnexpectedChar(c)));
                }
            }

            values.reverse();

            Ok(Chars {
                values,
                position: Position::start(),
            })
        }

        pub fn peek(&self) -> Option<char> {
            self.values.last().copied()
        }

        pub fn pop(&mut self) -> Option<char> {
            if let Some(c) = self.values.pop() {
                self.position.advance(c);
                Some(c)
            } else {
                None
            }
        }

        pub fn position(&self) -> Position {
            self.position
        }
    }
}

impl Chars {
    pub fn new(src: &str) -> Chars {
        Chars::try_new(src).unwrap()
    }

    pub fn try_take(&mut self, predicate: impl Fn(char) -> bool, buffer: &mut String) -> bool {
        self.take(predicate, buffer).is_ok()
    }

    pub fn take_while(&mut self, predicate: impl Fn(char) -> bool, buffer: &mut String) {
        while self.try_take(&predicate, buffer) {}
    }

    pub fn take(
        &mut self,
        predicate: impl Fn(char) -> bool,
        buffer: &mut String,
    ) -> Result<(), Error> {
        if let Some(c) = self.peek() {
            if predicate(c) {
                buffer.push(self.pop().unwrap());
                return Ok(());
            }
        }

        Err(Error::UnexpectedCharOrEndOfFile(self.peek()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::communication::{Position, PositionError};

    #[test]
    fn empty() {
        let mut chars = Chars::new("");

        assert_eq!(chars, Chars::try_new("").unwrap());

        assert!(chars.peek().is_none());
        assert!(chars.pop().is_none());

        assert!(chars.peek().is_none());
        assert!(chars.pop().is_none());

        assert!(chars.peek().is_none());
    }

    #[test]
    fn one_line() {
        let mut chars = Chars::new("a+b");

        assert_eq!(chars.peek(), Some('a'));
        assert_eq!(chars.pop(), Some('a'));

        assert_eq!(chars.peek(), Some('+'));
        assert_eq!(chars.pop(), Some('+'));

        assert_eq!(chars.peek(), Some('b'));
        assert_eq!(chars.pop(), Some('b'));

        assert!(chars.peek().is_none());
    }

    #[test]
    fn two_lines() {
        let mut chars = Chars::new("A\nB\n");

        assert_eq!(chars.pop(), Some('A'));
        assert_eq!(chars.pop(), Some('\n'));
        assert_eq!(chars.pop(), Some('B'));
        assert_eq!(chars.pop(), Some('\n'));

        assert!(chars.peek().is_none());
    }

    #[test]
    fn unexpected_char() {
        assert_eq!(
            Err(PositionError::new(
                Position::new(1, 2),
                Error::UnexpectedChar('\r')
            )),
            Chars::try_new("x\r"),
        );

        assert_eq!(
            Err(PositionError::new(
                Position::new(1, 2),
                Error::UnexpectedChar('\t')
            )),
            Chars::try_new("a\t\n"),
        );

        assert_eq!(
            Err(PositionError::new(
                Position::new(2, 1),
                Error::UnexpectedChar('\0')
            )),
            Chars::try_new("a\n\0\n"),
        );
    }
}
