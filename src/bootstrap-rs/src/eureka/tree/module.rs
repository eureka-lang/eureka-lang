use crate::communication::Error;
use crate::eureka::tree::{zero_or_more, PaddedDefinition};
use crate::eureka::{Padding, Tokens};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Module {
    pub pre_definitions_padding: Option<Padding>,
    pub definitions: Vec<PaddedDefinition>,
}

impl Module {
    pub fn parse(tokens: &mut Tokens) -> Result<Module, Error> {
        let pre_definitions_padding = tokens.try_take();
        let definitions = zero_or_more(PaddedDefinition::parse)(tokens)?;

        if let Some(token) = tokens.peek() {
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
    use crate::eureka::tree::{Definition, FunctionDefinition};
    use crate::eureka::{Identifier, Keyword, Padding};

    #[test]
    fn parse_empty() {
        let mut tokens = Tokens::new("");

        let actual = Module::parse(&mut tokens).unwrap();
        let expected = Module {
            pre_definitions_padding: None,
            definitions: Vec::new(),
        };

        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_success() {
        let mut tokens = Tokens::new("fn main() {}");

        let actual = Module::parse(&mut tokens).unwrap();
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
        let mut tokens = Tokens::new("fn main() {}return");

        let actual = Module::parse(&mut tokens).unwrap_err();
        let expected = Error::UnexpectedToken(Keyword::Return.into());

        assert_eq!(expected, actual);
    }
}
