use jscpd_json_parser_types::document::{JscpdDocument, JscpdParseState, JscpdSnapshot};

pub fn typed(document: &JscpdDocument) -> Option<&JscpdSnapshot> {
    match &document.typed {
        JscpdParseState::Parsed(snapshot) => Some(snapshot),
        JscpdParseState::Invalid(_) => None,
    }
}

pub fn parse_error_reason(document: &JscpdDocument) -> Option<&str> {
    match &document.typed {
        JscpdParseState::Parsed(_) => None,
        JscpdParseState::Invalid(reason) => Some(reason),
    }
}
