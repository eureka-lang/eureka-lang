use crate::communication::Error;
use crate::eureka::syntax_tree::FunctionDefinition;
use crate::eureka::Tokens;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Definition {
    Function(FunctionDefinition),
}

impl Definition {
    pub fn parse(tokens: &mut Tokens) -> Result<Option<Definition>, Error> {
        if let Some(definition) = FunctionDefinition::parse(tokens)? {
            Ok(Some(Definition::Function(definition)))
        } else {
            Ok(None)
        }
    }
}
