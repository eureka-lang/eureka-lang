use crate::communication::Error;
use crate::eureka::{Identifier, Keyword, Padding, Punctuator, Token, Tokens};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct FunctionDefinition {
    // Keyword::Fn
    pub pre_identifier_padding: Padding,
    pub identifier: Identifier,
    pub pre_parenthesis_padding: Option<Padding>,
    // Punctuator::LeftParenthesis
    // Punctuator::RightParenthesis
    pub pre_brace_padding: Option<Padding>,
    // Punctuator::LeftBrace
    // Punctuator::RightBrace
}

impl FunctionDefinition {
    pub fn parse(tokens: &mut Tokens) -> Result<Option<FunctionDefinition>, Error> {
        if tokens.peek() != Some(Token::Keyword(Keyword::Fn)) {
            return Ok(None);
        }

        tokens.pop();

        let pre_identifier_padding = tokens.take()?;
        let identifier = tokens.take()?;
        let pre_parenthesis_padding = tokens.try_take();
        tokens.expect(Punctuator::LeftParenthesis)?;
        tokens.expect(Punctuator::RightParenthesis)?;
        let pre_brace_padding = tokens.try_take();
        tokens.expect(Punctuator::LeftBrace)?;
        tokens.expect(Punctuator::RightBrace)?;

        Ok(Some(FunctionDefinition {
            pre_identifier_padding,
            identifier,
            pre_parenthesis_padding,
            pre_brace_padding,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::communication::Position;

    #[test]
    fn parse_success() {
        let mut tokens = Tokens::new("fn main() {}");
        let actual_function_definition = FunctionDefinition::parse(&mut tokens).unwrap().unwrap();
        let expected_function_definition = FunctionDefinition {
            pre_identifier_padding: Padding::new(" "),
            identifier: Identifier::new("main"),
            pre_parenthesis_padding: None,
            pre_brace_padding: Some(Padding::new(" ")),
        };
        assert_eq!(expected_function_definition, actual_function_definition);
    }

    #[test]
    fn parse_none() {
        let mut tokens = Tokens::new("return x");
        assert_eq!(Ok(None), FunctionDefinition::parse(&mut tokens));
    }

    #[test]
    fn parse_err() {
        let mut tokens = Tokens::new("fn main( {}");
        assert_eq!(tokens.position(), Position::new(1, 1));
        assert!(FunctionDefinition::parse(&mut tokens).is_err());
        assert_eq!(tokens.position(), Position::new(1, 9));
    }
}
