use crate::eureka::token::{Identifier, Padding};

mod parse;

struct SyntaxTree {
    definitions: Vec<FunctionDefinition>,
    post_definitions_padding: Option<Padding>,
}

struct FunctionDefinition {
    pre_keyword_padding: Option<Padding>,
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
