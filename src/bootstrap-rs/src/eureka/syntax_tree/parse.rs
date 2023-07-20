use crate::communication::{DisplayName, Error};
use crate::eureka::lexer::Lexer;
use crate::eureka::token::Token;

pub fn optional<T: TryFrom<Token>>(lexer: &mut Lexer) -> Option<T> {
    if let Some(token) = lexer.peek() {
        if let Ok(t) = T::try_from(token) {
            lexer.pop();
            return Some(t);
        }
    }

    None
}

pub fn required<T: DisplayName + TryFrom<Token>>(lexer: &mut Lexer) -> Result<T, Error> {
    optional::<T>(lexer).ok_or_else(|| Error::Missing(T::display_name()))
}

pub fn expected<T>(lexer: &mut Lexer, expected_token: T) -> Result<(), Error>
where
    T: Eq + PartialEq + Into<Token> + TryFrom<Token>,
{
    if let Some(token) = lexer.peek() {
        if let Ok(actual_token) = T::try_from(token) {
            if expected_token == actual_token {
                lexer.pop();
                return Ok(());
            }
        }
    }

    Err(Error::MissingToken(expected_token.into()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eureka::token::{Identifier, Keyword, Padding, Punctuator};

    #[test]
    fn test_optional_padding() {
        let mut lexer = Lexer::new(" fn");

        assert_eq!(lexer.peek(), Some(Token::Padding(Padding::new(" "))));

        let padding1 = optional::<Padding>(&mut lexer);

        assert_eq!(padding1, Some(Padding::new(" ")));
        assert_eq!(lexer.peek(), Some(Token::Keyword(Keyword::Fn)));

        let padding2 = optional::<Padding>(&mut lexer);

        assert_eq!(padding2, None);
        assert_eq!(lexer.peek(), Some(Token::Keyword(Keyword::Fn)));
    }

    #[test]
    fn test_required_padding() {
        let mut lexer = Lexer::new(" fn");

        assert_eq!(lexer.peek(), Some(Token::Padding(Padding::new(" "))));

        let padding1 = required::<Padding>(&mut lexer);

        assert_eq!(padding1, Ok(Padding::new(" ")));
        assert_eq!(lexer.peek(), Some(Token::Keyword(Keyword::Fn)));

        let padding2 = required::<Padding>(&mut lexer);

        assert_eq!(padding2, Err(Error::Missing("padding")));
        assert_eq!(lexer.peek(), Some(Token::Keyword(Keyword::Fn)));
    }

    #[test]
    fn test_expected() {
        let mut lexer = Lexer::new("fn(value");
        assert_eq!(lexer.peek(), Some(Keyword::Fn.into()));

        assert_eq!(
            Err(Error::MissingToken(Keyword::Return.into())),
            expected(&mut lexer, Keyword::Return),
        );
        assert_eq!(lexer.peek(), Some(Keyword::Fn.into()));

        assert_eq!(
            Err(Error::MissingToken(Identifier::new("value").into())),
            expected(&mut lexer, Identifier::new("value")),
        );
        assert_eq!(lexer.peek(), Some(Keyword::Fn.into()));

        assert_eq!(Ok(()), expected(&mut lexer, Keyword::Fn));
        assert_eq!(lexer.peek(), Some(Punctuator::LeftParenthesis.into()));

        assert_eq!(Ok(()), expected(&mut lexer, Punctuator::LeftParenthesis));
        assert_eq!(lexer.peek(), Some(Identifier::new("value").into()));

        assert_eq!(Ok(()), expected(&mut lexer, Identifier::new("value")));
        assert_eq!(lexer.peek(), None);

        assert_eq!(
            Err(Error::MissingToken(Punctuator::RightParenthesis.into())),
            expected(&mut lexer, Punctuator::RightParenthesis),
        );
        assert_eq!(lexer.peek(), None);
    }
}
