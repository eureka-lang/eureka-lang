pub fn lex_unquoted_name(src: &str) -> Option<(&str, &str)> {
    let mut chars = src.chars();
    let mut len = 0;

    if let Some(c) = chars.next() {
        if c == '_' || c.is_ascii_alphabetic() {
            len += c.len_utf8();

            while let Some(c) = chars.next() {
                if c == '_' || c.is_ascii_alphanumeric() {
                    len += c.len_utf8();
                } else {
                    break;
                }
            }
        }
    }

    if len > 0 {
        Some(src.split_at(len))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_unquoted_name_succeeds() {
        for (src, expected_name, expected_remaining_src) in [
            ("_,", "_", ","),
            ("T: ", "T", ": "),
            ("a+b", "a", "+b"),
            ("x1 - x2", "x1", " - x2"),
            ("y", "y", ""),
            (
                "abcdefghijklmnopqrstuvwxyz",
                "abcdefghijklmnopqrstuvwxyz",
                "",
            ),
            (
                "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
                "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
                "",
            ),
            ("_0123456789", "_0123456789", ""),
            ("__2__ _1_", "__2__", " _1_"),
            ("snake_case?", "snake_case", "?"),
            ("camelCase?", "camelCase", "?"),
            ("while true {}", "while", " true {}"),
        ] {
            let (actual_name, actual_remaining_src) = lex_unquoted_name(src).unwrap();

            assert_eq!(expected_name, actual_name);
            assert_eq!(expected_remaining_src, actual_remaining_src);
        }
    }

    #[test]
    fn lex_unquoted_name_fails() {
        for src in ["", "1", "2x", "-", "$", " ", "@", "[", "^", "`", "{"] {
            assert!(lex_unquoted_name(src).is_none());
        }
    }
}
