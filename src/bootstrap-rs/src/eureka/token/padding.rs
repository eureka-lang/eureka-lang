use crate::communication::{Error, INVALID_VALUE};
use crate::eureka::chars::Chars;
use crate::eureka::token::Token;
pub use restricted::Padding;

mod restricted {
    use crate::communication::Error;
    use crate::eureka::chars::Chars;

    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub struct Padding {
        value: String,
    }

    impl Padding {
        pub fn lex(chars: &mut Chars) -> Result<Option<Padding>, Error> {
            let mut value = String::new();

            while super::lex_whitespace(chars, &mut value) || super::lex_comment(chars, &mut value)?
            {
            }

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

fn lex_comment(chars: &mut Chars, buffer: &mut String) -> Result<bool, Error> {
    if chars.try_take(|c| c == '#', buffer) {
        chars.take_while(|c| matches!(c, ' '..='~'), buffer);
        chars.take(|c| c == '\n', buffer)?;

        Ok(true)
    } else {
        Ok(false)
    }
}

fn lex_whitespace(chars: &mut Chars, buffer: &mut String) -> bool {
    let buffer_len = buffer.len();
    chars.take_while(|c| c == ' ' || c == '\n', buffer);
    buffer.len() > buffer_len
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
            (" \n", " \n", None),
            (" else", " ", Some('e')),
            ("\n", "\n", None),
            ("\nSome ", "\n", Some('S')),
            ("#ok\n", "#ok\n", None),
            ("#ok\n2 \n", "#ok\n", Some('2')),
            (" \n\n ...", " \n\n ", Some('.')),
            (" #1\n?\n", " #1\n", Some('?')),
            ("#1\n ?\n", "#1\n ", Some('?')),
            (" \n\n#x\n", " \n\n#x\n", None),
            (
                "\n #1\n#2\n ##aA!~\n\n  \n\n#4\nSome \n\n#x\n",
                "\n #1\n#2\n ##aA!~\n\n  \n\n#4\n",
                Some('S'),
            ),
        ] {
            let mut chars = Chars::new(src);
            let actual_padding = Padding::lex(&mut chars).unwrap().unwrap();

            assert_eq!(expected_padding, actual_padding.unlex());
            assert_eq!(expected_peek, chars.peek());
        }
    }

    #[test]
    fn lex_fails() {
        for src in ["", "_", "-", "x", "1", "+"] {
            let mut chars = Chars::new(src);
            assert!(Padding::lex(&mut chars).unwrap().is_none());
            assert_eq!(src.chars().next(), chars.peek());
        }
    }

    #[test]
    fn lex_error() {
        let mut chars = Chars::new("#");
        assert!(Padding::lex(&mut chars).is_err());
    }

    #[test]
    fn test_lex_comment() {
        for (src, expected_buffer, expected_result) in [
            ("", "", Ok(false)),
            ("x\n", "", Ok(false)),
            ("#\n", "#\n", Ok(true)),
            ("#\nX", "#\n", Ok(true)),
            (
                "# This is a comment!\nY",
                "# This is a comment!\n",
                Ok(true),
            ),
            (
                "#a-zA-Z0-9?() !~ #ok\n    {\n",
                "#a-zA-Z0-9?() !~ #ok\n",
                Ok(true),
            ),
            ("## ## ##\n#\n", "## ## ##\n", Ok(true)),
        ] {
            let mut chars = Chars::new(src);
            let mut actual_buffer = String::new();

            assert_eq!(expected_result, lex_comment(&mut chars, &mut actual_buffer));
            assert_eq!(expected_buffer.to_string(), actual_buffer);
        }
    }

    #[test]
    fn test_lex_whitespace() {
        for (src, expected_buffer, expected_result) in [
            ("", "", false),
            ("x\n", "", false),
            (" ", " ", true),
            (" \n", " \n", true),
            (" x\n", " ", true),
            ("\n", "\n", true),
            ("\nFn ", "\n", true),
        ] {
            let mut chars = Chars::new(src);
            let mut actual_buffer = String::new();

            assert_eq!(
                expected_result,
                lex_whitespace(&mut chars, &mut actual_buffer),
            );
            assert_eq!(expected_buffer.to_string(), actual_buffer);
        }
    }
}
