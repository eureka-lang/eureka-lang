use crate::communication::{DisplayName, Error, INVALID_VALUE};
use crate::eureka::code::Code;
use crate::eureka::token::Token;
pub use restricted::Padding;
use std::fmt;

mod restricted {
    use crate::communication::Error;
    use crate::eureka::code::Code;

    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub struct Padding {
        value: String,
    }

    impl Padding {
        pub fn lex(src: &str) -> Option<(Padding, &str)> {
            let mut code = Code::new(src);

            if let Ok(Some(padding)) = Self::lex2(&mut code) {
                let len = padding.unlex().len();
                Some((padding, &src[len..]))
            } else {
                None
            }
        }

        pub fn lex2(code: &mut Code) -> Result<Option<Self>, Error> {
            let mut value = String::new();

            while super::lex_whitespace(code, &mut value) || super::lex_comment(code, &mut value)? {
            }

            if value.is_empty() {
                Ok(None)
            } else {
                Ok(Some(Self { value }))
            }
        }

        pub fn unlex(&self) -> &str {
            self.value.as_str()
        }
    }
}

fn lex_comment(code: &mut Code, buffer: &mut String) -> Result<bool, Error> {
    if let Some('#') = code.peek() {
        buffer.push(code.pop().unwrap());

        while let Some(' '..='~') = code.peek() {
            buffer.push(code.pop().unwrap());
        }

        if let Some('\n') = code.peek() {
            buffer.push(code.pop().unwrap());
            return Ok(true);
        } else {
            return Err(Error::UnexpectedCharOrEndOfFile(code.peek()));
        }
    }

    Ok(false)
}

fn lex_whitespace(code: &mut Code, buffer: &mut String) -> bool {
    let buffer_len = buffer.len();

    while let Some(' ' | '\n') = code.peek() {
        buffer.push(code.pop().unwrap());
    }

    buffer.len() > buffer_len
}

impl Padding {
    pub fn new(value: &str) -> Padding {
        let mut code = Code::new(value);

        if let Ok(Some(padding)) = Self::lex2(&mut code) {
            if code.peek().is_none() && padding.unlex() == value {
                return padding;
            }
        }

        panic!("{INVALID_VALUE}");
    }
}

impl fmt::Display for Padding {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!();
    }
}

impl DisplayName for Padding {
    fn display_name() -> &'static str {
        "padding"
    }
}

impl TryFrom<Token> for Padding {
    type Error = ();

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::Padding(padding) => Ok(padding),
            _ => Err(()),
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
        for (src, expected_padding, expected_remaining_src) in [
            (" ", " ", ""),
            (" \n", " \n", ""),
            (" else", " ", "else"),
            ("\n", "\n", ""),
            ("\nSome ", "\n", "Some "),
            ("#ok\n", "#ok\n", ""),
            ("#ok\n2 \n", "#ok\n", "2 \n"),
            (" \n\n ...", " \n\n ", "..."),
            (" #1\n?\n", " #1\n", "?\n"),
            ("#1\n ?\n", "#1\n ", "?\n"),
            (" \n\n#x\n", " \n\n#x\n", ""),
            (
                "\n #1\n#2\n ##aA!~\n\n  \n\n#4\nSome \n\n#x\n",
                "\n #1\n#2\n ##aA!~\n\n  \n\n#4\n",
                "Some \n\n#x\n",
            ),
        ] {
            let (actual_padding, actual_remaining_src) = Padding::lex(src).unwrap();

            assert_eq!(expected_padding, actual_padding.unlex());
            assert_eq!(expected_remaining_src, actual_remaining_src);
        }
    }

    #[test]
    fn lex_fails() {
        for src in ["", "_", "-", "x", "1", "+", "#"] {
            assert!(Padding::lex(src).is_none());
        }
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
            let mut code = Code::new(src);
            let mut actual_buffer = String::new();

            assert_eq!(expected_result, lex_comment(&mut code, &mut actual_buffer));
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
            let mut code = Code::new(src);
            let mut actual_buffer = String::new();

            assert_eq!(
                expected_result,
                lex_whitespace(&mut code, &mut actual_buffer),
            );
            assert_eq!(expected_buffer.to_string(), actual_buffer);
        }
    }
}
