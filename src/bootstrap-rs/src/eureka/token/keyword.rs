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

    pub fn lex(src: &str) -> Option<Keyword> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_is_sorted() {
        let mut map = Keyword::MAP.to_vec();

        map.sort_by_key(|&(key, _)| key);
        assert_eq!(map, Keyword::MAP.to_vec());

        map.sort_by_key(|&(_, key)| key);
        assert_eq!(map, Keyword::MAP.to_vec());
    }
}
