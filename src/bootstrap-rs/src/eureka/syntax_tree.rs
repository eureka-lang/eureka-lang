use crate::communication::Error;
use crate::eureka::lexer::Lexer;

pub use module::Module;
mod module;

pub use padded_definition::PaddedDefinition;
mod padded_definition;

pub use definition::Definition;
mod definition;

pub use function_definition::FunctionDefinition;
mod function_definition;

mod parse;

fn zero_or_more<T, F>(f: F) -> impl Fn(&mut Lexer) -> Result<Vec<T>, Error>
where
    F: Fn(&mut Lexer) -> Result<Option<T>, Error>,
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
    use crate::eureka::token::{Identifier, Padding};

    #[test]
    fn test_zero_or_more_parse_function_definition_zero() {
        let mut lexer = Lexer::new("");

        let actual = zero_or_more(FunctionDefinition::parse)(&mut lexer);
        let expected: Vec<FunctionDefinition> = Vec::new();

        assert_eq!(expected, actual.unwrap());
    }

    #[test]
    fn test_zero_or_more_parse_function_definition_one() {
        let mut lexer = Lexer::new("fn main() {}");

        let actual = zero_or_more(FunctionDefinition::parse)(&mut lexer);
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
        let mut lexer = Lexer::new("fn a(){}fn b(){}");

        let actual = zero_or_more(FunctionDefinition::parse)(&mut lexer);
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
        let mut lexer = Lexer::new("fn main( {}");
        assert_eq!(lexer.position(), Position::new(1, 1));
        assert!(zero_or_more(FunctionDefinition::parse)(&mut lexer).is_err());
        assert_eq!(lexer.position(), Position::new(1, 9));
    }
}
