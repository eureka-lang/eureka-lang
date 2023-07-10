use crate::eureka::lexer::Lexer;
use crate::eureka::token::{Identifier, Keyword, Padding, Punctuator, Token};

mod parse;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Module {
    pre_definitions_padding: Option<Padding>,
    definitions: Vec<PaddedDefinition>,
}

fn parse_module(lexer: &mut Lexer) -> Result<Module, String> {
    let pre_definitions_padding = parse::optional(lexer);
    let definitions = zero_or_more(parse_padded_definition)(lexer)?;

    if let Some(token) = lexer.peek() {
        return Err(format!("unexpected token: {}", token));
    }

    Ok(Module {
        pre_definitions_padding,
        definitions,
    })
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct PaddedDefinition {
    definition: Definition,
    post_definition_padding: Option<Padding>,
}

fn parse_padded_definition(lexer: &mut Lexer) -> Result<Option<PaddedDefinition>, String> {
    if let Some(definition) = parse_definition(lexer)? {
        let post_definition_padding = parse::optional(lexer);

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

fn parse_definition(lexer: &mut Lexer) -> Result<Option<Definition>, String> {
    if let Some(definition) = parse_function_definition(lexer)? {
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

fn parse_function_definition(lexer: &mut Lexer) -> Result<Option<FunctionDefinition>, String> {
    if lexer.peek() != Some(&Token::Keyword(Keyword::Fn)) {
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

fn zero_or_more<T, F>(f: F) -> impl Fn(&mut Lexer) -> Result<Vec<T>, String>
where
    F: Fn(&mut Lexer) -> Result<Option<T>, String>,
{
    move |lexer: &mut Lexer| {
        let mut result = Vec::new();

        while let Some(t) = f(lexer)? {
            result.push(t);
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::communication::Position;

    #[test]
    fn test_parse_function_definition_success() {
        let mut lexer = Lexer::lex_all("fn main() {}").unwrap();
        let actual_function_definition = parse_function_definition(&mut lexer).unwrap().unwrap();
        let expected_function_definition = FunctionDefinition {
            pre_identifier_padding: Padding::new(" "),
            identifier: Identifier::new("main"),
            pre_parenthesis_padding: None,
            pre_brace_padding: Some(Padding::new(" ")),
        };
        assert_eq!(expected_function_definition, actual_function_definition);
    }

    #[test]
    fn test_parse_function_definition_err() {
        let mut lexer = Lexer::lex_all("fn main( {}").unwrap();
        assert_eq!(lexer.position(), Position::new(1, 1));
        assert!(parse_function_definition(&mut lexer).is_err());
        assert_eq!(lexer.position(), Position::new(1, 9));
    }

    #[test]
    fn test_parse_function_definition_none() {
        let mut lexer = Lexer::lex_all("return x").unwrap();
        assert_eq!(Ok(None), parse_function_definition(&mut lexer));
    }

    #[test]
    fn test_zero_or_more_parse_function_definition_zero() {
        let mut lexer = Lexer::lex_all("").unwrap();

        let actual = zero_or_more(parse_function_definition)(&mut lexer);
        let expected: Vec<FunctionDefinition> = Vec::new();

        assert_eq!(expected, actual.unwrap());
    }

    #[test]
    fn test_zero_or_more_parse_function_definition_one() {
        let mut lexer = Lexer::lex_all("fn main() {}").unwrap();

        let actual = zero_or_more(parse_function_definition)(&mut lexer);
        let expected: Vec<FunctionDefinition> = vec![FunctionDefinition {
            pre_identifier_padding: Padding::new(" "),
            identifier: Identifier::new("main"),
            pre_parenthesis_padding: None,
            pre_brace_padding: Some(Padding::new(" ")),
        }];

        assert_eq!(expected, actual.unwrap());
    }

    #[test]
    fn test_zero_or_more_parse_function_definition_two() {
        let mut lexer = Lexer::lex_all("fn a(){}fn b(){}").unwrap();

        let actual = zero_or_more(parse_function_definition)(&mut lexer);
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
    fn test_zero_or_more_parse_function_definition_err() {
        let mut lexer = Lexer::lex_all("fn main( {}").unwrap();
        assert_eq!(lexer.position(), Position::new(1, 1));
        assert!(zero_or_more(parse_function_definition)(&mut lexer).is_err());
        assert_eq!(lexer.position(), Position::new(1, 9));
    }

    #[test]
    fn test_parse_module_empty() {
        let mut lexer = Lexer::lex_all("").unwrap();

        let actual = parse_module(&mut lexer).unwrap();
        let expected = Module {
            pre_definitions_padding: None,
            definitions: Vec::new(),
        };

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_module_err() {
        let mut lexer = Lexer::lex_all("fn main() {}return").unwrap();

        let actual = parse_module(&mut lexer).unwrap_err();
        let expected = "unexpected token: \"return\"";

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_module_success() {
        let mut lexer = Lexer::lex_all("fn main() {}").unwrap();

        let actual = parse_module(&mut lexer).unwrap();
        let expected = Module {
            pre_definitions_padding: None,
            definitions: vec![PaddedDefinition {
                definition: Definition::Function(FunctionDefinition {
                    pre_identifier_padding: Padding::new(" "),
                    identifier: Identifier::new("main"),
                    pre_parenthesis_padding: None,
                    pre_brace_padding: Some(Padding::new(" ")),
                }),
                post_definition_padding: None,
            }],
        };

        assert_eq!(expected, actual);
    }
}
