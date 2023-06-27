use crate::eureka::token::{Token, Tokens};
use crate::miscellaneous::{missing, DisplayName};

pub fn optional<T: TryFrom<Token>>(tokens: &mut Tokens) -> Option<T> {
    if let Some(token) = tokens.peek().cloned() {
        if let Ok(t) = T::try_from(token) {
            tokens.pop();
            return Some(t);
        }
    }

    None
}

pub fn required<T: DisplayName + TryFrom<Token>>(tokens: &mut Tokens) -> Result<T, String> {
    optional::<T>(tokens).ok_or_else(|| missing(T::display_name()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eureka::token::{Keyword, Padding};

    #[test]
    fn test_optional_padding() {
        let mut tokens = Tokens::lex_all(" fn").unwrap();

        assert_eq!(tokens.peek(), Some(&Token::Padding(Padding::new(" "))));

        let padding1 = optional::<Padding>(&mut tokens);

        assert_eq!(padding1, Some(Padding::new(" ")));
        assert_eq!(tokens.peek(), Some(&Token::Keyword(Keyword::Fn)));

        let padding2 = optional::<Padding>(&mut tokens);

        assert_eq!(padding2, None);
        assert_eq!(tokens.peek(), Some(&Token::Keyword(Keyword::Fn)));
    }

    #[test]
    fn test_required_padding() {
        let mut tokens = Tokens::lex_all(" fn").unwrap();

        assert_eq!(tokens.peek(), Some(&Token::Padding(Padding::new(" "))));

        let padding1 = required::<Padding>(&mut tokens);

        assert_eq!(padding1, Ok(Padding::new(" ")));
        assert_eq!(tokens.peek(), Some(&Token::Keyword(Keyword::Fn)));

        let padding2 = required::<Padding>(&mut tokens);

        assert_eq!(padding2, Err("missing padding".to_string()));
        assert_eq!(tokens.peek(), Some(&Token::Keyword(Keyword::Fn)));
    }
}
