use crate::eureka::token::{Identifier, Keyword, Padding, Punctuator, Token, Tokens};

mod parse;

struct SyntaxTree {
    pre_definitions_padding: Option<Padding>,
    definitions: Vec<PaddedDefinition>,
}

struct PaddedDefinition {
    definition: Definition,
    post_definition_padding: Option<Padding>,
}

enum Definition {
    Function(FunctionDefinition),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct FunctionDefinition {
    // Keyword::Fn
    pre_identifier_padding: Padding,
    identifier: Identifier,
    pre_parenthesis_padding: Option<Padding>,
    // Punctuator::LeftParenthesis
    // Punctuator::RightParenthesis
    pre_brace_padding: Option<Padding>,
    // Punctuator::LeftBrace
    // Punctuator::RightBrace
}

fn parse_optional_function_definition(
    tokens: &mut Tokens,
) -> Result<Option<FunctionDefinition>, String> {
    if tokens.peek() != Some(&Token::Keyword(Keyword::Fn)) {
        return Ok(None);
    }

    tokens.pop();

    let pre_identifier_padding = parse::required(tokens)?;
    let identifier = parse::required(tokens)?;
    let pre_parenthesis_padding = parse::optional(tokens);
    parse::expected(tokens, Punctuator::LeftParenthesis)?;
    parse::expected(tokens, Punctuator::RightParenthesis)?;
    let pre_brace_padding = parse::optional(tokens);
    parse::expected(tokens, Punctuator::LeftBrace)?;
    parse::expected(tokens, Punctuator::RightBrace)?;

    Ok(Some(FunctionDefinition {
        pre_identifier_padding,
        identifier,
        pre_parenthesis_padding,
        pre_brace_padding,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::text::Position;

    #[test]
    fn test_parse_optional_function_definition_success() {
        let mut tokens = Tokens::lex_all("fn main() {}").unwrap();
        let actual_function_definition = parse_optional_function_definition(&mut tokens)
            .unwrap()
            .unwrap();
        let expected_function_definition = FunctionDefinition {
            pre_identifier_padding: Padding::new(" "),
            identifier: Identifier::new("main"),
            pre_parenthesis_padding: None,
            pre_brace_padding: Some(Padding::new(" ")),
        };
        assert_eq!(expected_function_definition, actual_function_definition);
    }

    #[test]
    fn test_parse_optional_function_definition_err() {
        let mut tokens = Tokens::lex_all("fn main( {}").unwrap();
        assert_eq!(tokens.position(), Position::new(1, 1));
        assert!(parse_optional_function_definition(&mut tokens).is_err());
        assert_eq!(tokens.position(), Position::new(1, 9));
    }

    #[test]
    fn test_parse_optional_function_definition_none() {
        let mut tokens = Tokens::lex_all("return x").unwrap();
        assert_eq!(Ok(None), parse_optional_function_definition(&mut tokens));
    }
}
