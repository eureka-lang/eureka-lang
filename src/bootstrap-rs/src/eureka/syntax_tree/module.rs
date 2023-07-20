use crate::communication::Error;
use crate::eureka::lexer::Lexer;
use crate::eureka::syntax_tree::{parse, zero_or_more, PaddedDefinition};
use crate::eureka::token::Padding;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Module {
    pub pre_definitions_padding: Option<Padding>,
    pub definitions: Vec<PaddedDefinition>,
}

impl Module {
    pub fn parse(lexer: &mut Lexer) -> Result<Module, Error> {
        let pre_definitions_padding = parse::optional(lexer);
        let definitions = zero_or_more(PaddedDefinition::parse)(lexer)?;

        if let Some(token) = lexer.peek() {
            return Err(Error::UnexpectedToken(token));
        }

        Ok(Module {
            pre_definitions_padding,
            definitions,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eureka::syntax_tree::{Definition, FunctionDefinition};
    use crate::eureka::token::{Identifier, Keyword, Padding};

    #[test]
    fn parse_empty() {
        let mut lexer = Lexer::new("");

        let actual = Module::parse(&mut lexer).unwrap();
        let expected = Module {
            pre_definitions_padding: None,
            definitions: Vec::new(),
        };

        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_success() {
        let mut lexer = Lexer::new("fn main() {}");

        let actual = Module::parse(&mut lexer).unwrap();
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

    #[test]
    fn parse_err() {
        let mut lexer = Lexer::new("fn main() {}return");

        let actual = Module::parse(&mut lexer).unwrap_err();
        let expected = Error::UnexpectedToken(Keyword::Return.into());

        assert_eq!(expected, actual);
    }
}
