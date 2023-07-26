use crate::{eureka, language};

pub type Chars = language::Chars<eureka::Char>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::communication::{Error, Position, PositionError};

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
