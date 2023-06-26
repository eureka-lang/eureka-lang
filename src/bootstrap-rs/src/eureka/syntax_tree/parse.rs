use crate::eureka::token::{Padding, Token, Tokens};

pub fn optional_padding(tokens: &mut Tokens) -> Option<Padding> {
    if let Some(Token::Padding(padding)) = tokens.peek().cloned() {
        tokens.pop();
        Some(padding)
    } else {
        None
    }
}

pub fn padding(tokens: &mut Tokens) -> Result<Padding, String> {
    optional_padding(tokens).ok_or_else(|| "missing padding".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eureka::token::Keyword;

    #[test]
    fn test_optional_padding() {
        let mut tokens = Tokens::lex_all(" fn").unwrap();

        assert_eq!(tokens.peek(), Some(&Token::Padding(Padding::new(" "))));

        let padding1 = optional_padding(&mut tokens);

        assert_eq!(padding1, Some(Padding::new(" ")));
        assert_eq!(tokens.peek(), Some(&Token::Keyword(Keyword::Fn)));

        let padding2 = optional_padding(&mut tokens);

        assert_eq!(padding2, None);
        assert_eq!(tokens.peek(), Some(&Token::Keyword(Keyword::Fn)));
    }

    #[test]
    fn test_padding() {
        let mut tokens = Tokens::lex_all(" fn").unwrap();

        assert_eq!(tokens.peek(), Some(&Token::Padding(Padding::new(" "))));

        let padding1 = padding(&mut tokens);

        assert_eq!(padding1, Ok(Padding::new(" ")));
        assert_eq!(tokens.peek(), Some(&Token::Keyword(Keyword::Fn)));

        let padding2 = padding(&mut tokens);

        assert_eq!(padding2, Err("missing padding".to_string()));
        assert_eq!(tokens.peek(), Some(&Token::Keyword(Keyword::Fn)));
    }
}
