use crate::communication::Error;
use crate::eureka::lexer::Lexer;
use crate::eureka::syntax_tree::FunctionDefinition;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Definition {
    Function(FunctionDefinition),
}

impl Definition {
    pub fn parse(lexer: &mut Lexer) -> Result<Option<Definition>, Error> {
        if let Some(definition) = FunctionDefinition::parse(lexer)? {
            Ok(Some(Definition::Function(definition)))
        } else {
            Ok(None)
        }
    }
}
