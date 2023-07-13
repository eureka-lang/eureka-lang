use crate::communication::PositionError;
pub use restricted::Code;

mod restricted {
    use crate::communication::Error::{MissingChar, UnexpectedChar};
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

            if !chars.is_empty() && chars.last().copied() != Some('\n') {
                return Err(PositionError::new(position, MissingChar('\n')));
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

    pub fn normalize(src: &str) -> Result<Code, PositionError> {
        let mut src = src.replace("\r\n", "\n");

        if !src.is_empty() && src.chars().last() != Some('\n') {
            src.push('\n');
        }

        Code::try_new(&src)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::communication::{Error, Position};

    #[test]
    fn empty() {
        let mut code = Code::new("");

        assert_eq!(code, Code::try_new("").unwrap());
        assert_eq!(code, Code::normalize("").unwrap());

        assert!(code.peek().is_none());
        assert!(code.pop().is_none());

        assert!(code.peek().is_none());
        assert!(code.pop().is_none());

        assert!(code.peek().is_none());
    }

    #[test]
    fn normalize_one_line() {
        for src in ["a+b", "a+b\n", "a+b\r\n"] {
            let mut code = Code::normalize(src).unwrap();

            assert_eq!(code.peek(), Some('a'));
            assert_eq!(code.pop(), Some('a'));

            assert_eq!(code.peek(), Some('+'));
            assert_eq!(code.pop(), Some('+'));

            assert_eq!(code.peek(), Some('b'));
            assert_eq!(code.pop(), Some('b'));

            assert_eq!(code.peek(), Some('\n'));
            assert_eq!(code.pop(), Some('\n'));

            assert!(code.peek().is_none());
        }
    }

    #[test]
    fn normalize_two_lines() {
        for src in [
            "A\nB",
            "A\nB\n",
            "A\nB\r\n",
            "A\r\nB",
            "A\r\nB\n",
            "A\r\nB\r\n",
        ] {
            let mut code = Code::normalize(src).unwrap();

            assert_eq!(code.pop(), Some('A'));
            assert_eq!(code.pop(), Some('\n'));
            assert_eq!(code.pop(), Some('B'));
            assert_eq!(code.pop(), Some('\n'));

            assert!(code.peek().is_none());
        }
    }

    #[test]
    fn normalizing_carriage_return_at_end_of_file_fails() {
        assert_eq!(
            Err(PositionError::new(
                Position::new(1, 2),
                Error::UnexpectedChar('\r')
            )),
            Code::normalize("x\r"),
        );
    }

    #[test]
    fn unexpected_char() {
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

    #[test]
    fn missing_newline_at_end_of_file() {
        assert_eq!(
            Err(PositionError::new(
                Position::new(1, 2),
                Error::MissingChar('\n')
            )),
            Code::try_new("a"),
        );
    }
}
