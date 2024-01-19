use crate::communication::{Error, INVALID_VALUE};
use crate::eureka::{Chars, Token};
pub use restricted::Padding;

mod restricted {
    use crate::communication::Error;
    use crate::eureka::Chars;

    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub struct Padding {
        value: String,
    }

    impl Padding {
        pub fn lex(chars: &mut Chars) -> Result<Option<Padding>, Error> {
            let mut value = String::new();

            chars.take_while(|c| c == ' ' || c == '\n', &mut value);

            if value.is_empty() {
                Ok(None)
            } else {
                Ok(Some(Padding { value }))
            }
        }

        pub fn unlex(&self) -> &str {
            self.value.as_str()
        }
    }
}

impl Padding {
    pub fn new(value: &str) -> Padding {
        let mut chars = Chars::new(value);

        if let Ok(Some(padding)) = Self::lex(&mut chars) {
            if chars.peek().is_none() {
                return padding;
            }
        }

        panic!("{INVALID_VALUE}");
    }
}

impl TryFrom<Option<Token>> for Padding {
    type Error = Error;

    fn try_from(value: Option<Token>) -> Result<Self, Self::Error> {
        match value {
            Some(Token::Padding(padding)) => Ok(padding),
            _ => Err(Error::Expected("padding")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_succeeds() {
        let padding = Padding::new(" ");
        assert_eq!(" ", padding.unlex());
    }

    #[test]
    #[should_panic(expected = "invalid value")]
    fn new_fails_if_value_does_not_start_with_padding() {
        let _ = Padding::new("x");
    }

    #[test]
    #[should_panic(expected = "invalid value")]
    fn new_fails_if_value_is_not_entirely_padding() {
        let _ = Padding::new(" x");
    }

    #[test]
    fn lex_succeeds() {
        for (src, expected_padding, expected_peek) in [
            (" ", " ", None),
            (" else", " ", Some('e')),
            ("\n", "\n", None),
            ("\nSome ", "\n", Some('S')),
            (" \n", " \n", None),
            (" \n\n ...", " \n\n ", Some('.')),
        ] {
            let mut chars = Chars::new(src);
            let actual_padding = Padding::lex(&mut chars).unwrap().unwrap();

            assert_eq!(expected_padding, actual_padding.unlex());
            assert_eq!(expected_peek, chars.peek());
        }
    }

    #[test]
    fn lex_fails() {
        for src in ["", "_", "-", "x", "x\n", "1", "+", "#", "#\n"] {
            let mut chars = Chars::new(src);
            assert!(Padding::lex(&mut chars).unwrap().is_none());
            assert_eq!(src.chars().next(), chars.peek());
        }
    }
}
