use eslint_directive_parser_types::document::{
    EslintDirectiveDocument, EslintDirectiveFileState, EslintDirectiveParseState,
};

pub fn typed(document: &EslintDirectiveDocument) -> &EslintDirectiveFileState {
    &document.typed
}

pub fn parse_error_reason(document: &EslintDirectiveDocument) -> Option<&str> {
    match &document.typed.state {
        EslintDirectiveParseState::ParseError { reason }
        | EslintDirectiveParseState::Unsupported { reason }
        | EslintDirectiveParseState::Ambiguous { reason } => Some(reason),
        EslintDirectiveParseState::Parsed { .. } => None,
    }
}
