use crate::communication::Error;
use crate::eureka::lexer::Lexer;
use crate::eureka::syntax_tree::{parse, Definition};
use crate::eureka::token::Padding;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct PaddedDefinition {
    pub definition: Definition,
    pub post_definition_padding: Option<Padding>,
}

impl PaddedDefinition {
    pub fn parse(lexer: &mut Lexer) -> Result<Option<PaddedDefinition>, Error> {
        if let Some(definition) = Definition::parse(lexer)? {
            let post_definition_padding = parse::optional(lexer);

            Ok(Some(PaddedDefinition {
                definition,
                post_definition_padding,
            }))
        } else {
            Ok(None)
        }
    }
}
