use npmrc_parser_types::document::{NpmrcDocument, NpmrcParseState, NpmrcSnapshot};

#[must_use]
pub const fn typed(document: &NpmrcDocument) -> Option<&NpmrcSnapshot> {
    match &document.typed {
        NpmrcParseState::Parsed(snapshot) => Some(snapshot),
        NpmrcParseState::Invalid(_) => None,
    }
}

#[must_use]
pub fn parse_error_reason(document: &NpmrcDocument) -> Option<&str> {
    match &document.typed {
        NpmrcParseState::Parsed(_) => None,
        NpmrcParseState::Invalid(reason) => Some(reason),
    }
}

#[must_use]
pub fn effective_value<'a>(document: &'a NpmrcDocument, key: &str) -> Option<&'a str> {
    typed(document).and_then(|snapshot| {
        snapshot
            .settings
            .iter()
            .rev()
            .find(|setting| setting.key == key)
            .map(|setting| setting.value.as_str())
    })
}
