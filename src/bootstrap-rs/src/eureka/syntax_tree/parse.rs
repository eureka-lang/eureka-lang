use crate::eureka::lexer::Lexer;
use crate::eureka::token::Token;
use crate::miscellaneous::{missing, DisplayName};
use std::fmt;

pub fn optional<T: TryFrom<Token>>(lexer: &mut Lexer) -> Option<T> {
    if let Some(token) = lexer.peek().cloned() {
        if let Ok(t) = T::try_from(token) {
            lexer.pop();
            return Some(t);
        }
    }

    None
}

pub fn required<T: DisplayName + TryFrom<Token>>(lexer: &mut Lexer) -> Result<T, String> {
    optional::<T>(lexer).ok_or_else(|| missing(T::display_name()))
}

pub fn expected<T>(lexer: &mut Lexer, expected_token: T) -> Result<(), String>
where
    T: Eq + PartialEq + fmt::Display + DisplayName + TryFrom<Token>,
{
    if let Some(token) = lexer.peek().cloned() {
        if let Ok(actual_token) = T::try_from(token) {
            if expected_token == actual_token {
                lexer.pop();
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
        let mut lexer = Lexer::lex_all(" fn").unwrap();

        assert_eq!(lexer.peek(), Some(&Token::Padding(Padding::new(" "))));

        let padding1 = optional::<Padding>(&mut lexer);

        assert_eq!(padding1, Some(Padding::new(" ")));
        assert_eq!(lexer.peek(), Some(&Token::Keyword(Keyword::Fn)));

        let padding2 = optional::<Padding>(&mut lexer);

        assert_eq!(padding2, None);
        assert_eq!(lexer.peek(), Some(&Token::Keyword(Keyword::Fn)));
    }

    #[test]
    fn test_required_padding() {
        let mut lexer = Lexer::lex_all(" fn").unwrap();

        assert_eq!(lexer.peek(), Some(&Token::Padding(Padding::new(" "))));

        let padding1 = required::<Padding>(&mut lexer);

        assert_eq!(padding1, Ok(Padding::new(" ")));
        assert_eq!(lexer.peek(), Some(&Token::Keyword(Keyword::Fn)));

        let padding2 = required::<Padding>(&mut lexer);

        assert_eq!(padding2, Err("missing padding".to_string()));
        assert_eq!(lexer.peek(), Some(&Token::Keyword(Keyword::Fn)));
    }

    #[test]
    fn test_expected() {
        let mut lexer = Lexer::lex_all("fn(value").unwrap();
        assert_eq!(lexer.peek(), Some(&Keyword::Fn.into()));

        assert_eq!(
            Err("missing keyword: \"return\"".to_string()),
            expected(&mut lexer, Keyword::Return),
        );
        assert_eq!(lexer.peek(), Some(&Keyword::Fn.into()));

        assert_eq!(
            Err("missing identifier: \"value\"".to_string()),
            expected(&mut lexer, Identifier::new("value")),
        );
        assert_eq!(lexer.peek(), Some(&Keyword::Fn.into()));

        assert_eq!(Ok(()), expected(&mut lexer, Keyword::Fn));
        assert_eq!(lexer.peek(), Some(&Punctuator::LeftParenthesis.into()));

        assert_eq!(Ok(()), expected(&mut lexer, Punctuator::LeftParenthesis));
        assert_eq!(lexer.peek(), Some(&Identifier::new("value").into()));

        assert_eq!(Ok(()), expected(&mut lexer, Identifier::new("value")));
        assert_eq!(lexer.peek(), None);

        assert_eq!(
            Err("missing punctuator: \")\"".to_string()),
            expected(&mut lexer, Punctuator::RightParenthesis),
        );
        assert_eq!(lexer.peek(), None);
    }
}
