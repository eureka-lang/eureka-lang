use super::lex;
use crate::eureka::token::Token;
use crate::miscellaneous::DisplayName;
use crate::text::Position;
pub use restricted::Padding;

mod restricted {
    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub struct Padding {
        value: String,
    }

    impl Padding {
        pub fn as_str(&self) -> &str {
            self.value.as_str()
        }

        pub fn lex(src: &str) -> Option<(Padding, &str)> {
            let mut remaining_src;

            if let Some(new_remaining_src) = super::skip_whitespace(src) {
                remaining_src = new_remaining_src;
            } else if let Some(new_remaining_src) = super::skip_comment(src) {
                remaining_src = new_remaining_src;
            } else {
                return None;
            }

            loop {
                if let Some(new_remaining_src) = super::skip_whitespace(remaining_src) {
                    remaining_src = new_remaining_src;
                } else if let Some(new_remaining_src) = super::skip_comment(remaining_src) {
                    remaining_src = new_remaining_src;
                } else {
                    break;
                }
            }

            let len = src.len() - remaining_src.len();
            assert!(len > 0);

            let padding = Padding {
                value: String::from(&src[..len]),
            };

            Some((padding, remaining_src))
        }
    }
}

fn skip_comment(src: &str) -> Option<&str> {
    let mut chars = src.chars();

    if let Some('#') = chars.next() {
        while let Some(c) = chars.next() {
            if c.is_ascii_graphic() || c == ' ' || c == '\t' {
                continue;
            } else if c == '\n' {
                return Some(chars.as_str());
            } else if c == '\r' && chars.next() == Some('\n') {
                return Some(chars.as_str());
            } else {
                break;
            }
        }
    }

    None
}

fn skip_whitespace(src: &str) -> Option<&str> {
    let mut chars = src.chars();

    if let Some(c) = chars.next() {
        if c == ' ' || c == '\t' || c == '\n' {
            return Some(chars.as_str());
        } else if c == '\r' && chars.next() == Some('\n') {
            return Some(chars.as_str());
        }
    }

    None
}

impl Padding {
    pub fn new(value: &str) -> Padding {
        lex::entirely(Padding::lex)(value)
    }

    pub fn relative_end(&self) -> Position {
        let mut line_count = 0;
        let mut previous_line = "";

        for line in self.as_str().split('\n') {
            line_count += 1;
            previous_line = line;
        }

        Position::new(line_count, previous_line.len() + 1)
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
        assert_eq!(" ", padding.as_str());
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
            (" else", " ", "else"),
            ("\t", "\t", ""),
            ("\t{", "\t", "{"),
            ("\n", "\n", ""),
            ("\nSome ", "\n", "Some "),
            ("\r\n", "\r\n", ""),
            ("\r\nNone\t", "\r\n", "None\t"),
            ("#ok\n", "#ok\n", ""),
            ("#ok\n2 ", "#ok\n", "2 "),
            ("#ok\r\n", "#ok\r\n", ""),
            ("#ok\r\n2 ", "#ok\r\n", "2 "),
            (" \t\n\r\n\n\t ...", " \t\n\r\n\n\t ", "..."),
            (" #1\n?", " #1\n", "?"),
            ("#1\n ?", "#1\n ", "?"),
            (" \t\n\r\n#x\n", " \t\n\r\n#x\n", ""),
            (
                "\n #1\n\t#2\r\n ##aA!~\n\n  \t\t\r\n\r\n#4\r\nSome \t\n\r\n#x\n",
                "\n #1\n\t#2\r\n ##aA!~\n\n  \t\t\r\n\r\n#4\r\n",
                "Some \t\n\r\n#x\n",
            ),
        ] {
            let (actual_padding, actual_remaining_src) = Padding::lex(src).unwrap();

            assert_eq!(expected_padding, actual_padding.as_str());
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
    fn test_skip_comment() {
        for (src, expected_result) in [
            ("", None),
            ("x", None),
            ("#", None),
            ("#\x1B", None),
            ("#\x1B\n", None),
            ("#x\x1B", None),
            ("#x\x1B\r\n", None),
            ("#\r", None),
            ("#\rX\n", None),
            ("#\r \n", None),
            ("#\r\t\n", None),
            ("#\r\r\n", None),
            ("#\0\n", None),
            ("#\0\r\n", None),
            ("#\n", Some("")),
            ("#\nX", Some("X")),
            ("#\r\n", Some("")),
            ("#\r\nX", Some("X")),
            ("# This is a comment!\nY", Some("Y")),
            ("#a-zA-Z0-9?()\t !~ #ok\r\n    {", Some("    {")),
            ("## ## ##\n#\n", Some("#\n")),
        ] {
            assert_eq!(expected_result, skip_comment(src));
        }
    }

    #[test]
    fn test_skip_whitespace() {
        for (src, expected_result) in [
            ("", None),
            ("x", None),
            ("\r", None),
            ("\rX", None),
            ("\r ", None),
            ("\r X", None),
            ("\r\t", None),
            ("\r\r", None),
            (" ", Some("")),
            (" x", Some("x")),
            ("\t", Some("")),
            ("\t{ ", Some("{ ")),
            ("\n", Some("")),
            ("\nFn ", Some("Fn ")),
            ("\r\n", Some("")),
            ("\r\nFn ", Some("Fn ")),
        ] {
            assert_eq!(expected_result, skip_whitespace(src));
        }
    }

    #[test]
    fn padding_relative_end() {
        assert_eq!(Padding::new(" ").relative_end(), Position::new(1, 2));
        assert_eq!(Padding::new("\t\t").relative_end(), Position::new(1, 3));
        assert_eq!(Padding::new("\n").relative_end(), Position::new(2, 1));
        assert_eq!(Padding::new("\r\n").relative_end(), Position::new(2, 1));
        assert_eq!(Padding::new("\n\n").relative_end(), Position::new(3, 1));
        assert_eq!(Padding::new("\r\n\r\n").relative_end(), Position::new(3, 1));
        assert_eq!(Padding::new("\t\t\n ").relative_end(), Position::new(2, 2));
        assert_eq!(
            Padding::new(" \r\n\t\t").relative_end(),
            Position::new(2, 3),
        );
        assert_eq!(
            Padding::new(" \n#c\n    ").relative_end(),
            Position::new(3, 5),
        );
        assert_eq!(
            Padding::new("#c\r\n\t\r\n\t").relative_end(),
            Position::new(3, 2),
        );
    }
}
