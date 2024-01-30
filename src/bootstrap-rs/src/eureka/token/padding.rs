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
            let mut level: u8 = 0;

            loop {
                match chars.peek() {
                    Some(' ' | '\n') => value.push(chars.pop().unwrap()),
                    Some('/') if chars.peek2() == Some('*') => {
                        level = level
                            .checked_add(1)
                            .ok_or(Error::ExceededMaximumNestingLevel)?;
                        value.push(chars.pop().unwrap());
                        value.push(chars.pop().unwrap());
                    }
                    Some('*') if chars.peek2() == Some('/') && level > 0 => {
                        level -= 1;
                        value.push(chars.pop().unwrap());
                        value.push(chars.pop().unwrap());
                    }
                    Some('!'..='~') if level > 0 => value.push(chars.pop().unwrap()),
                    _ => break,
                }
            }

            if level > 0 {
                Err(Error::Expected("*/"))
            } else if value.is_empty() {
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
    fn new() {
        let padding = Padding::new(" ");
        assert_eq!(" ", padding.unlex());
    }

    #[test]
    #[should_panic(expected = "invalid value")]
    fn new_panics_if_value_does_not_start_with_padding() {
        let _ = Padding::new("x");
    }

    #[test]
    #[should_panic(expected = "invalid value")]
    fn new_panics_if_value_is_not_entirely_padding() {
        let _ = Padding::new(" x");
    }

    #[test]
    fn lex_some() {
        for (src, expected_padding, expected_peek) in [
            (" ", " ", None),
            (" else", " ", Some('e')),
            ("\n", "\n", None),
            ("\nSome ", "\n", Some('S')),
            ("/**/", "/**/", None),
            ("/**/*", "/**/", Some('*')),
            ("/**//", "/**/", Some('/')),
            ("/**/ */", "/**/ ", Some('*')),
            ("/** */ /**/", "/** */ /**/", None),
            ("/*/ */", "/*/ */", None),
            (" /* !0-9*A_Z/a~z\n*/\n", " /* !0-9*A_Z/a~z\n*/\n", None),
            (" \n\n /* /**/ */ ...", " \n\n /* /**/ */ ", Some('.')),
        ] {
            let mut chars = Chars::new(src);
            let actual_padding = Padding::lex(&mut chars).unwrap().unwrap();

            assert_eq!(expected_padding, actual_padding.unlex());
            assert_eq!(expected_peek, chars.peek());
        }
    }

    #[test]
    fn lex_none() {
        for src in [
            "", "_", "-", "x", "x\n", "1", "+", "!", "~", "#", "#\n", "*/", "*/**/", "//**/",
        ] {
            let mut chars = Chars::new(src);
            assert!(Padding::lex(&mut chars).unwrap().is_none());
            assert_eq!(src.chars().next(), chars.peek());
        }
    }

    #[test]
    fn lex_error() {
        for (src, expected_column) in [("/*", 3), ("/*/", 4)] {
            let mut chars = Chars::new(src);
            assert!(Padding::lex(&mut chars).is_err());
            assert_eq!(1, chars.position().line());
            assert_eq!(expected_column, chars.position().column());
        }
    }

    #[test]
    fn lex_maximum_nesting_level() {
        let mut src = String::new();
        src.extend(std::iter::repeat("/*").take(255));
        src.extend(std::iter::repeat("*/").take(255));

        let mut chars = Chars::new(&src);
        let padding = Padding::lex(&mut chars).unwrap().unwrap();

        assert_eq!(src.as_str(), padding.unlex());
        assert!(chars.peek().is_none());
    }

    #[test]
    fn lex_error_exceeded_maximum_nesting_level() {
        let mut src = String::new();
        src.extend(std::iter::repeat("/*").take(256));
        src.extend(std::iter::repeat("*/").take(256));

        let mut chars = Chars::new(&src);
        assert_eq!(
            Error::ExceededMaximumNestingLevel,
            Padding::lex(&mut chars).unwrap_err(),
        );
        assert_eq!(1, chars.position().line());
        assert_eq!(511, chars.position().column());
    }
}
