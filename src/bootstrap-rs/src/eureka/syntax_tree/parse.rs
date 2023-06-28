use crate::eureka::token::{Token, Tokens};
use crate::miscellaneous::{missing, DisplayName};
use std::fmt;

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

pub fn expected<T>(tokens: &mut Tokens, expected_token: T) -> Result<(), String>
where
    T: Eq + PartialEq + fmt::Display + DisplayName + TryFrom<Token>,
{
    if let Some(token) = tokens.peek().cloned() {
        if let Ok(actual_token) = T::try_from(token) {
            if expected_token == actual_token {
                tokens.pop();
                return Ok(());
            }
        }
    }

    Err(missing(&format!(
        "{}: {}",
        T::display_name(),
        expected_token.to_string(),
    )))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eureka::token::{Identifier, Keyword, Padding, Punctuator};

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

    #[test]
    fn test_expected() {
        let mut tokens = Tokens::lex_all("fn(value").unwrap();
        assert_eq!(tokens.peek(), Some(&Keyword::Fn.into()));

        assert_eq!(
            Err("missing keyword: return".to_string()),
            expected(&mut tokens, Keyword::Return),
        );
        assert_eq!(tokens.peek(), Some(&Keyword::Fn.into()));

        assert_eq!(
            Err("missing identifier: value".to_string()),
            expected(&mut tokens, Identifier::new("value")),
        );
        assert_eq!(tokens.peek(), Some(&Keyword::Fn.into()));

        assert_eq!(Ok(()), expected(&mut tokens, Keyword::Fn));
        assert_eq!(tokens.peek(), Some(&Punctuator::LeftParenthesis.into()));

        assert_eq!(Ok(()), expected(&mut tokens, Punctuator::LeftParenthesis));
        assert_eq!(tokens.peek(), Some(&Identifier::new("value").into()));

        assert_eq!(Ok(()), expected(&mut tokens, Identifier::new("value")));
        assert_eq!(tokens.peek(), None);

        assert_eq!(
            Err("missing punctuator: )".to_string()),
            expected(&mut tokens, Punctuator::RightParenthesis),
        );
        assert_eq!(tokens.peek(), None);
    }
}
