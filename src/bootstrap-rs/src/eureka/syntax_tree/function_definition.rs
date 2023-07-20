use crate::communication::Error;
use crate::eureka::lexer::Lexer;
use crate::eureka::syntax_tree::parse;
use crate::eureka::token::{Identifier, Keyword, Padding, Punctuator, Token};

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
    pub fn parse(lexer: &mut Lexer) -> Result<Option<FunctionDefinition>, Error> {
        if lexer.peek() != Some(Token::Keyword(Keyword::Fn)) {
            return Ok(None);
        }

        lexer.pop();

        let pre_identifier_padding = parse::required(lexer)?;
        let identifier = parse::required(lexer)?;
        let pre_parenthesis_padding = parse::optional(lexer);
        parse::expected(lexer, Punctuator::LeftParenthesis)?;
        parse::expected(lexer, Punctuator::RightParenthesis)?;
        let pre_brace_padding = parse::optional(lexer);
        parse::expected(lexer, Punctuator::LeftBrace)?;
        parse::expected(lexer, Punctuator::RightBrace)?;

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
        let mut lexer = Lexer::new("fn main() {}");
        let actual_function_definition = FunctionDefinition::parse(&mut lexer).unwrap().unwrap();
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
        let mut lexer = Lexer::new("return x");
        assert_eq!(Ok(None), FunctionDefinition::parse(&mut lexer));
    }

    #[test]
    fn parse_err() {
        let mut lexer = Lexer::new("fn main( {}");
        assert_eq!(lexer.position(), Position::new(1, 1));
        assert!(FunctionDefinition::parse(&mut lexer).is_err());
        assert_eq!(lexer.position(), Position::new(1, 9));
    }
}
