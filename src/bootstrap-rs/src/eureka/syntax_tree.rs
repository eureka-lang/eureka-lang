use crate::eureka::token::{Identifier, Keyword, Padding, Punctuator, Token, Tokens};

mod parse;

struct SyntaxTree {
    module_body: ModuleBody,
}

struct ModuleBody {
    pre_definitions_padding: Option<Padding>,
    definitions: Vec<PaddedDefinition>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct PaddedDefinition {
    definition: Definition,
    post_definition_padding: Option<Padding>,
}

fn parse_optional_padded_definition(
    tokens: &mut Tokens,
) -> Result<Option<PaddedDefinition>, String> {
    if let Some(definition) = parse_optional_definition(tokens)? {
        let post_definition_padding = parse::optional(tokens);

        Ok(Some(PaddedDefinition {
            definition,
            post_definition_padding,
        }))
    } else {
        Ok(None)
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Definition {
    Function(FunctionDefinition),
}

fn parse_optional_definition(tokens: &mut Tokens) -> Result<Option<Definition>, String> {
    if let Some(definition) = parse_optional_function_definition(tokens)? {
        Ok(Some(Definition::Function(definition)))
    } else {
        Ok(None)
    }
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

fn zero_or_more<T, F>(f: F) -> impl Fn(&mut Tokens) -> Result<Vec<T>, String>
where
    F: Fn(&mut Tokens) -> Result<Option<T>, String>,
{
    move |tokens: &mut Tokens| {
        let mut result = Vec::new();

        while let Some(t) = f(tokens)? {
            result.push(t);
        }

        Ok(result)
    }
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

    #[test]
    fn test_zero_or_more_parse_optional_function_definition_zero() {
        let mut tokens = Tokens::lex_all("").unwrap();

        let actual = zero_or_more(parse_optional_function_definition)(&mut tokens);
        let expected: Vec<FunctionDefinition> = Vec::new();

        assert_eq!(expected, actual.unwrap());
    }

    #[test]
    fn test_zero_or_more_parse_optional_function_definition_one() {
        let mut tokens = Tokens::lex_all("fn main() {}").unwrap();

        let actual = zero_or_more(parse_optional_function_definition)(&mut tokens);
        let expected: Vec<FunctionDefinition> = vec![FunctionDefinition {
            pre_identifier_padding: Padding::new(" "),
            identifier: Identifier::new("main"),
            pre_parenthesis_padding: None,
            pre_brace_padding: Some(Padding::new(" ")),
        }];

        assert_eq!(expected, actual.unwrap());
    }

    #[test]
    fn test_zero_or_more_parse_optional_function_definition_two() {
        let mut tokens = Tokens::lex_all("fn a(){}fn b(){}").unwrap();

        let actual = zero_or_more(parse_optional_function_definition)(&mut tokens);
        let expected: Vec<FunctionDefinition> = vec![
            FunctionDefinition {
                pre_identifier_padding: Padding::new(" "),
                identifier: Identifier::new("a"),
                pre_parenthesis_padding: None,
                pre_brace_padding: None,
            },
            FunctionDefinition {
                pre_identifier_padding: Padding::new(" "),
                identifier: Identifier::new("b"),
                pre_parenthesis_padding: None,
                pre_brace_padding: None,
            },
        ];

        assert_eq!(expected, actual.unwrap());
    }

    #[test]
    fn test_zero_or_more_parse_optional_function_definition_err() {
        let mut tokens = Tokens::lex_all("fn main( {}").unwrap();
        assert_eq!(tokens.position(), Position::new(1, 1));
        assert!(zero_or_more(parse_optional_function_definition)(&mut tokens).is_err());
        assert_eq!(tokens.position(), Position::new(1, 9));
    }
}
