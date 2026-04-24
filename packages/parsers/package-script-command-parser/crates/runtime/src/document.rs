use package_script_command_parser_types::document::{
    PackageScriptCommandDocument, PackageScriptParseFact, PackageScriptParseState,
};

pub fn typed(document: &PackageScriptCommandDocument) -> &PackageScriptParseFact {
    &document.typed
}

pub fn parse_error_reason(document: &PackageScriptCommandDocument) -> Option<&str> {
    match &document.typed.state {
        PackageScriptParseState::Unsupported { reason }
        | PackageScriptParseState::ParseError { reason } => Some(reason),
        PackageScriptParseState::Parsed { .. } | PackageScriptParseState::NoEslintInvocation => {
            None
        }
    }
}
