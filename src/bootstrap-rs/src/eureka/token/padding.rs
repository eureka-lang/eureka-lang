use crate::eureka::code::Code;
use crate::eureka::token::Token;
use crate::miscellaneous::DisplayName;
use crate::text::Position;
pub use restricted::Padding;
use std::fmt;

mod restricted {
    use crate::eureka::code::Code;

    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub struct Padding {
        value: String,
    }

    impl Padding {
        pub fn lex(src: &str) -> Option<(Padding, &str)> {
            let mut code = Code::new(src);

            if let Ok(Some(padding)) = Self::lex2(&mut code) {
                let mut r_count = 0;
                loop {
                    let new_r_count = src
                        .chars()
                        .take(r_count + padding.unlex().len())
                        .filter(|&c| c == '\r')
                        .count();
                    if new_r_count == r_count {
                        break;
                    }
                    r_count = new_r_count;
                }
                let len = (r_count + padding.unlex().len()).min(src.len());
                Some((padding, &src[len..]))
            } else {
                None
            }
        }

        pub fn lex2(code: &mut Code) -> Result<Option<Self>, String> {
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

fn lex_comment(code: &mut Code, buffer: &mut String) -> Result<bool, String> {
    if let Some('#') = code.peek() {
        buffer.push(code.pop().unwrap());

        while let Some(' '..='~' | '\t') = code.peek() {
            buffer.push(code.pop().unwrap());
        }

        if let Some('\n') = code.peek() {
            buffer.push(code.pop().unwrap());
            return Ok(true);
        } else {
            return Err(format!("unexpected: {:?}", code.peek()));
        }
    }

    Ok(false)
}

fn lex_whitespace(code: &mut Code, buffer: &mut String) -> bool {
    let buffer_len = buffer.len();

    while let Some(' ' | '\t' | '\n') = code.peek() {
        buffer.push(code.pop().unwrap());
    }

    buffer.len() > buffer_len
}

impl Padding {
    pub fn new(value: &str) -> Padding {
        let mut code = Code::new(&format!("{value};\n"));
        if let Ok(Some(padding)) = Self::lex2(&mut code) {
            if padding.unlex() == value {
                return padding;
            }
        }

        panic!("invalid value");
    }

    pub fn relative_end(&self) -> Position {
        let mut line_count = 0;
        let mut previous_line = "";

        for line in self.unlex().split('\n') {
            line_count += 1;
            previous_line = line;
        }

        Position::new(line_count, previous_line.len() + 1)
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
            (" ", " \n", ""),
            (" else", " ", "else"),
            ("\t", "\t\n", ""),
            ("\t{", "\t", "{"),
            ("\n", "\n", ""),
            ("\nSome ", "\n", "Some "),
            ("\r\n", "\n", ""),
            ("\r\nNone\t", "\n", "None\t"),
            ("#ok", "#ok\n", ""),
            ("#ok\n", "#ok\n", ""),
            ("#ok\n2 ", "#ok\n", "2 "),
            ("#ok\r\n", "#ok\n", ""),
            ("#ok\r\n2 ", "#ok\n", "2 "),
            (" \t\n\r\n\n\t ...", " \t\n\n\n\t ", "..."),
            (" #1\n?", " #1\n", "?"),
            ("#1\n ?", "#1\n ", "?"),
            (" \t\n\r\n#x\n", " \t\n\n#x\n", ""),
            (
                "\n #1\n\t#2\r\n ##aA!~\n\n  \t\t\r\n\r\n#4\r\nSome \t\n\r\n#x\n",
                "\n #1\n\t#2\n ##aA!~\n\n  \t\t\n\n#4\n",
                "Some \t\n\r\n#x\n",
            ),
        ] {
            let (actual_padding, actual_remaining_src) = Padding::lex(src).unwrap();

            assert_eq!(expected_padding, actual_padding.unlex());
            assert_eq!(expected_remaining_src, actual_remaining_src);
        }
    }

    #[test]
    fn lex_fails() {
        for src in ["", "_", "-", "x", "1", "+", "\r", "\r ", "#\r", "#\r "] {
            assert!(Padding::lex(src).is_none());
        }
    }

    #[test]
    fn test_lex_comment() {
        for (src, expected_buffer, expected_result) in [
            ("", "", Ok(false)),
            ("x\n", "", Ok(false)),
            ("#\n", "#\n", Ok(true)),
            ("#\nX\n", "#\n", Ok(true)),
            (
                "# This is a comment!\nY\n",
                "# This is a comment!\n",
                Ok(true),
            ),
            (
                "#a-zA-Z0-9?()\t !~ #ok\n    {\n",
                "#a-zA-Z0-9?()\t !~ #ok\n",
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
            ("\rX\n", "", false),
            ("\r \n", "", false),
            ("\r X\n", "", false),
            ("\r\t\n", "", false),
            ("\r\r\n", "", false),
            (" \n", " \n", true),
            (" x\n", " ", true),
            ("\t\n", "\t\n", true),
            ("\t{ \n", "\t", true),
            ("\n", "\n", true),
            ("\nFn \n", "\n", true),
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

    #[test]
    fn padding_relative_end() {
        assert_eq!(Padding::new(" ").relative_end(), Position::new(1, 2));
        assert_eq!(Padding::new("\t\t").relative_end(), Position::new(1, 3));
        assert_eq!(Padding::new("\n").relative_end(), Position::new(2, 1));
        assert_eq!(Padding::new("\n\n").relative_end(), Position::new(3, 1));
        assert_eq!(Padding::new("\t\t\n ").relative_end(), Position::new(2, 2));
        assert_eq!(Padding::new(" \n\t\t").relative_end(), Position::new(2, 3));
        assert_eq!(
            Padding::new(" \n#c\n    ").relative_end(),
            Position::new(3, 5),
        );
        assert_eq!(
            Padding::new("#c\n\t\n\t").relative_end(),
            Position::new(3, 2),
        );
    }
}
