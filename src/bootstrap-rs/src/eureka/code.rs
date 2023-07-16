pub use restricted::Code;

mod restricted {
    use crate::communication::Error::UnexpectedChar;
    use crate::communication::{Position, PositionError};

    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub struct Code {
        chars: Vec<char>,
        position: Position,
    }

    impl Code {
        pub fn try_new(src: &str) -> Result<Code, PositionError> {
            let mut chars: Vec<char> = Vec::with_capacity(src.len());
            let mut position = Position::start();

            for c in src.chars() {
                if (' ' <= c && c <= '~') || c == '\n' {
                    chars.push(c);
                    position.advance(c);
                } else {
                    return Err(PositionError::new(position, UnexpectedChar(c)));
                }
            }

            chars.reverse();

            Ok(Code {
                chars,
                position: Position::start(),
            })
        }

        pub fn peek(&self) -> Option<char> {
            self.chars.last().copied()
        }

        pub fn pop(&mut self) -> Option<char> {
            if let Some(c) = self.chars.pop() {
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

impl Code {
    pub fn new(src: &str) -> Code {
        Code::try_new(src).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::communication::{Error, Position, PositionError};

    #[test]
    fn empty() {
        let mut code = Code::new("");

        assert_eq!(code, Code::try_new("").unwrap());

        assert!(code.peek().is_none());
        assert!(code.pop().is_none());

        assert!(code.peek().is_none());
        assert!(code.pop().is_none());

        assert!(code.peek().is_none());
    }

    #[test]
    fn one_line() {
        let mut code = Code::new("a+b");

        assert_eq!(code.peek(), Some('a'));
        assert_eq!(code.pop(), Some('a'));

        assert_eq!(code.peek(), Some('+'));
        assert_eq!(code.pop(), Some('+'));

        assert_eq!(code.peek(), Some('b'));
        assert_eq!(code.pop(), Some('b'));

        assert!(code.peek().is_none());
    }

    #[test]
    fn two_lines() {
        let mut code = Code::new("A\nB\n");

        assert_eq!(code.pop(), Some('A'));
        assert_eq!(code.pop(), Some('\n'));
        assert_eq!(code.pop(), Some('B'));
        assert_eq!(code.pop(), Some('\n'));

        assert!(code.peek().is_none());
    }

    #[test]
    fn unexpected_char() {
        assert_eq!(
            Err(PositionError::new(
                Position::new(1, 2),
                Error::UnexpectedChar('\r')
            )),
            Code::try_new("x\r"),
        );

        assert_eq!(
            Err(PositionError::new(
                Position::new(1, 2),
                Error::UnexpectedChar('\t')
            )),
            Code::try_new("a\t\n"),
        );

        assert_eq!(
            Err(PositionError::new(
                Position::new(2, 1),
                Error::UnexpectedChar('\0')
            )),
            Code::try_new("a\n\0\n"),
        );
    }
}
