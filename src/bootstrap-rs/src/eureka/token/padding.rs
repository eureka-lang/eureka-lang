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

        if let Some(new_remaining_src) = skip_whitespace(src) {
            remaining_src = new_remaining_src;
        } else if let Some(new_remaining_src) = skip_comment(src) {
            remaining_src = new_remaining_src;
        } else {
            return None;
        }

        loop {
            if let Some(new_remaining_src) = skip_whitespace(remaining_src) {
                remaining_src = new_remaining_src;
            } else if let Some(new_remaining_src) = skip_comment(remaining_src) {
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
