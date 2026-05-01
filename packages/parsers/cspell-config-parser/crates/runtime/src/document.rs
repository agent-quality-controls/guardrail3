use cspell_config_parser_types::document::{
    CspellConfigDocument, CspellConfigParseState, CspellConfigSnapshot,
};

#[must_use]
pub fn typed(document: &CspellConfigDocument) -> Option<&CspellConfigSnapshot> {
    match &document.typed {
        CspellConfigParseState::Parsed(snapshot) => Some(snapshot),
        CspellConfigParseState::Invalid(_) => None,
    }
}

#[must_use]
pub fn parse_error_reason(document: &CspellConfigDocument) -> Option<&str> {
    match &document.typed {
        CspellConfigParseState::Invalid(reason) => Some(reason),
        CspellConfigParseState::Parsed(_) => None,
    }
}
