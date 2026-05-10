use stylelint_config_parser_types::document::{
    StylelintConfigDocument, StylelintConfigParseState, StylelintConfigSnapshot,
    StylelintEffectiveConfigProbe,
};

#[must_use]
pub const fn typed(document: &StylelintConfigDocument) -> Option<&StylelintConfigSnapshot> {
    match &document.typed {
        StylelintConfigParseState::Parsed(snapshot) => Some(snapshot),
        StylelintConfigParseState::Invalid(_) => None,
    }
}

#[must_use]
pub fn parse_error_reason(document: &StylelintConfigDocument) -> Option<&str> {
    match &document.typed {
        StylelintConfigParseState::Parsed(_) => None,
        StylelintConfigParseState::Invalid(reason) => Some(reason),
    }
}

#[must_use]
pub fn probe(document: &StylelintConfigDocument) -> Option<&StylelintEffectiveConfigProbe> {
    typed(document)?.probes.first()
}

#[must_use]
pub fn rule_setting<'a>(
    document: &'a StylelintConfigDocument,
    rule_name: &str,
) -> Option<&'a serde_json::Value> {
    probe(document)?.rules.get(rule_name)
}
