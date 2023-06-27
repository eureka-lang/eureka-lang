use super::name::lex_unquoted_name;
use crate::eureka::token::Token;
use crate::miscellaneous::DisplayName;
use crate::text::Position;

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

    const REVERSE_MAP: [(Keyword, &str); 3] = [
        (Keyword::Fn, "fn"),
        (Keyword::If, "if"),
        (Keyword::Return, "return"),
    ];

    pub fn lex(src: &str) -> Option<(Keyword, &str)> {
        if let Some((name, remaining_src)) = lex_unquoted_name(src) {
            if let Ok(index) = Keyword::MAP.binary_search_by_key(&name, |&(key, _)| key) {
                return Some((Keyword::MAP[index].1, remaining_src));
            }
        }

        None
    }

    pub fn len(&self) -> usize {
        let index = Keyword::REVERSE_MAP
            .binary_search_by_key(self, |&(key, _)| key)
            .unwrap();

        Keyword::REVERSE_MAP[index].1.len()
    }

    pub fn relative_end(&self) -> Position {
        Position::new(1, self.len() + 1)
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
    }

    #[test]
    fn reverse_map_is_sorted() {
        let mut map = Keyword::REVERSE_MAP.to_vec();
        map.sort_by_key(|&(key, _)| key);
        assert_eq!(map, Keyword::REVERSE_MAP.to_vec());
    }

    #[test]
    fn reverse_map_is_consistent_with_map() {
        assert_eq!(Keyword::MAP.len(), Keyword::REVERSE_MAP.len());

        let zip = Keyword::MAP.iter().zip(Keyword::REVERSE_MAP.iter());
        for ((s1, k1), (k2, s2)) in zip {
            assert_eq!(s1, s2);
            assert_eq!(k1, k2);
        }
    }

    #[test]
    fn keyword_len() {
        assert_eq!(Keyword::Fn.len(), 2);
        assert_eq!(Keyword::Return.len(), 6);
    }

    #[test]
    fn relative_end() {
        assert_eq!(Keyword::Fn.relative_end(), Position::new(1, 3));
        assert_eq!(Keyword::Return.relative_end(), Position::new(1, 7));
    }
}
