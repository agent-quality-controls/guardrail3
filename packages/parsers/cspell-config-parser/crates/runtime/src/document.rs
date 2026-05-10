use cspell_config_parser_types::document::{
    CspellConfigDocument, CspellConfigParseState, CspellConfigSnapshot,
};

/// Returns the parsed snapshot when the document parsed successfully, otherwise `None`.
#[must_use]
pub const fn typed(document: &CspellConfigDocument) -> Option<&CspellConfigSnapshot> {
    if let CspellConfigParseState::Parsed(snapshot) = &document.typed {
        Some(snapshot)
    } else {
        None
    }
}

/// Returns the captured parse-error reason when the document failed to parse, otherwise `None`.
#[must_use]
pub fn parse_error_reason(document: &CspellConfigDocument) -> Option<&str> {
    if let CspellConfigParseState::Invalid(reason) = &document.typed {
        Some(reason.as_str())
    } else {
        None
    }
}
