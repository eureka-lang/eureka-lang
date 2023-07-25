use crate::communication::Error;
use crate::eureka::syntax_tree::Definition;
use crate::eureka::token::Padding;
use crate::eureka::tokens::Tokens;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct PaddedDefinition {
    pub definition: Definition,
    pub post_definition_padding: Option<Padding>,
}

impl PaddedDefinition {
    pub fn parse(tokens: &mut Tokens) -> Result<Option<PaddedDefinition>, Error> {
        if let Some(definition) = Definition::parse(tokens)? {
            let post_definition_padding = tokens.try_take();

            Ok(Some(PaddedDefinition {
                definition,
                post_definition_padding,
            }))
        } else {
            Ok(None)
        }
    }
}
