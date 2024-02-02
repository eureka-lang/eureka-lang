use crate::communication::Error;
use crate::eureka::tree::{zero_or_more, Definition};
use crate::eureka::{Padding, Tokens};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Module {
    pub definitions: Vec<Definition>,
    pub post_definitions_padding: Option<Padding>,
}

impl Module {
    pub fn parse(tokens: &mut Tokens) -> Result<Module, Error> {
        let definitions = zero_or_more(Definition::parse)(tokens)?;
        let post_definitions_padding = tokens.try_take();

        if let Some(token) = tokens.peek() {
            return Err(Error::UnexpectedToken(token));
        }

        Ok(Module {
            definitions,
            post_definitions_padding,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eureka::tree::{Definition, FunctionDefinition};
    use crate::eureka::{Identifier, Keyword};

    #[test]
    fn parse_empty() {
        let mut tokens = Tokens::new("");

        let actual = Module::parse(&mut tokens).unwrap();
        let expected = Module {
            definitions: Vec::new(),
            post_definitions_padding: None,
        };

        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_non_empty() {
        let mut tokens = Tokens::new(" fn main() {}\n");

        let actual = Module::parse(&mut tokens).unwrap();
        let expected = Module {
            definitions: vec![Definition::Function(FunctionDefinition {
                pre_definition_padding: Some(Padding::new(" ")),
                pre_identifier_padding: Padding::new(" "),
                identifier: Identifier::new("main"),
                pre_parenthesis_padding: None,
                pre_brace_padding: Some(Padding::new(" ")),
            })],
            post_definitions_padding: Some(Padding::new("\n")),
        };

        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_error() {
        let mut tokens = Tokens::new("fn main() {}return");

        let actual = Module::parse(&mut tokens).unwrap_err();
        let expected = Error::UnexpectedToken(Keyword::Return.into());

        assert_eq!(expected, actual);
    }
}
