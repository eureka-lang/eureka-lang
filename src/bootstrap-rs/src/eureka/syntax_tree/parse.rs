use crate::communication::{DisplayName, Error};
use crate::eureka::token::Token;
use crate::eureka::tokens::Tokens;

pub fn optional<T: TryFrom<Token>>(tokens: &mut Tokens) -> Option<T> {
    if let Some(token) = tokens.peek() {
        if let Ok(t) = T::try_from(token) {
            tokens.pop();
            return Some(t);
        }
    }

    None
}

pub fn required<T: DisplayName + TryFrom<Token>>(tokens: &mut Tokens) -> Result<T, Error> {
    optional::<T>(tokens).ok_or_else(|| Error::Missing(T::display_name()))
}

pub fn expected<T>(tokens: &mut Tokens, expected_token: T) -> Result<(), Error>
where
    T: Eq + PartialEq + Into<Token> + TryFrom<Token>,
{
    if let Some(token) = tokens.peek() {
        if let Ok(actual_token) = T::try_from(token) {
            if expected_token == actual_token {
                tokens.pop();
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
        let mut tokens = Tokens::new(" fn");

        assert_eq!(tokens.peek(), Some(Token::Padding(Padding::new(" "))));

        let padding1 = optional::<Padding>(&mut tokens);

        assert_eq!(padding1, Some(Padding::new(" ")));
        assert_eq!(tokens.peek(), Some(Token::Keyword(Keyword::Fn)));

        let padding2 = optional::<Padding>(&mut tokens);

        assert_eq!(padding2, None);
        assert_eq!(tokens.peek(), Some(Token::Keyword(Keyword::Fn)));
    }

    #[test]
    fn test_required_padding() {
        let mut tokens = Tokens::new(" fn");

        assert_eq!(tokens.peek(), Some(Token::Padding(Padding::new(" "))));

        let padding1 = required::<Padding>(&mut tokens);

        assert_eq!(padding1, Ok(Padding::new(" ")));
        assert_eq!(tokens.peek(), Some(Token::Keyword(Keyword::Fn)));

        let padding2 = required::<Padding>(&mut tokens);

        assert_eq!(padding2, Err(Error::Missing("padding")));
        assert_eq!(tokens.peek(), Some(Token::Keyword(Keyword::Fn)));
    }

    #[test]
    fn test_expected() {
        let mut tokens = Tokens::new("fn(value");
        assert_eq!(tokens.peek(), Some(Keyword::Fn.into()));

        assert_eq!(
            Err(Error::MissingToken(Keyword::Return.into())),
            expected(&mut tokens, Keyword::Return),
        );
        assert_eq!(tokens.peek(), Some(Keyword::Fn.into()));

        assert_eq!(
            Err(Error::MissingToken(Identifier::new("value").into())),
            expected(&mut tokens, Identifier::new("value")),
        );
        assert_eq!(tokens.peek(), Some(Keyword::Fn.into()));

        assert_eq!(Ok(()), expected(&mut tokens, Keyword::Fn));
        assert_eq!(tokens.peek(), Some(Punctuator::LeftParenthesis.into()));

        assert_eq!(Ok(()), expected(&mut tokens, Punctuator::LeftParenthesis));
        assert_eq!(tokens.peek(), Some(Identifier::new("value").into()));

        assert_eq!(Ok(()), expected(&mut tokens, Identifier::new("value")));
        assert_eq!(tokens.peek(), None);

        assert_eq!(
            Err(Error::MissingToken(Punctuator::RightParenthesis.into())),
            expected(&mut tokens, Punctuator::RightParenthesis),
        );
        assert_eq!(tokens.peek(), None);
    }
}
