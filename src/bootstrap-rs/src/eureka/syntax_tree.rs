use crate::eureka::token::{Identifier, Padding};

mod parse;

struct SyntaxTree {
    definitions: Vec<PaddedDefinition>,
    post_definitions_padding: Option<Padding>,
}

struct PaddedDefinition {
    pre_definition_padding: Option<Padding>,
    definition: Definition,
}

enum Definition {
    Function(FunctionDefinition),
}

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
