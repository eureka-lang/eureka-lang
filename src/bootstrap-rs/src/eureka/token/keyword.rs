use super::name::lex_unquoted_name;
use crate::communication::DisplayName;
use crate::eureka::token::Token;
use std::fmt;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Keyword {
    Fn,
    If,
    Return,
}

impl Keyword {
    const MAP: [(&str, Keyword); 3] = [
        ("fn", Keyword::Fn),
        ("if", Keyword::If),
        ("return", Keyword::Return),
    ];

    pub fn lex(src: &str) -> Option<(Keyword, &str)> {
        if let Some((name, remaining_src)) = lex_unquoted_name(src) {
            if let Ok(index) = Keyword::MAP.binary_search_by_key(&name, |&(key, _)| key) {
                return Some((Keyword::MAP[index].1, remaining_src));
            }
        }

        None
    }

    pub fn lex2(src: &str) -> Option<Keyword> {
        if let Ok(index) = Keyword::MAP.binary_search_by_key(&src, |&(key, _)| key) {
            Some(Keyword::MAP[index].1)
        } else {
            None
        }
    }

    pub fn unlex(&self) -> &'static str {
        let index = Keyword::MAP
            .binary_search_by_key(self, |&(_, key)| key)
            .unwrap();

        Keyword::MAP[index].0
    }
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", self.unlex())
    }
}

impl DisplayName for Keyword {
    fn display_name() -> &'static str {
        "keyword"
    }
}

impl TryFrom<Token> for Keyword {
    type Error = ();

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::Keyword(keyword) => Ok(keyword),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_succeeds() {
        for (src, expected_keyword, expected_remaining_src) in [
            ("fn", Keyword::Fn, ""),
            ("fn main", Keyword::Fn, " main"),
            ("if a < b {}", Keyword::If, " a < b {}"),
            ("return 0;", Keyword::Return, " 0;"),
        ] {
            let (actual_keyword, actual_remaining_src) = Keyword::lex(src).unwrap();

            assert_eq!(expected_keyword, actual_keyword);
            assert_eq!(expected_remaining_src, actual_remaining_src);
        }
    }

    #[test]
    fn lex_fails() {
        for src in ["", "_", "fnX", "fn2", "#if", "+"] {
            assert!(Keyword::lex(src).is_none());
        }
    }

    #[test]
    fn map_is_sorted() {
        let mut map = Keyword::MAP.to_vec();

        map.sort_by_key(|&(key, _)| key);
        assert_eq!(map, Keyword::MAP.to_vec());

        map.sort_by_key(|&(_, key)| key);
        assert_eq!(map, Keyword::MAP.to_vec());
    }
}
