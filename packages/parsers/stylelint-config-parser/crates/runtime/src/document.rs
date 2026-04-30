use stylelint_config_parser_types::document::{
    StylelintConfigDocument, StylelintConfigParseState, StylelintConfigSnapshot,
    StylelintEffectiveConfigProbe,
};

pub fn typed(document: &StylelintConfigDocument) -> Option<&StylelintConfigSnapshot> {
    match &document.typed {
        StylelintConfigParseState::Parsed(snapshot) => Some(snapshot),
        StylelintConfigParseState::Invalid(_) => None,
    }
}

pub fn parse_error_reason(document: &StylelintConfigDocument) -> Option<&str> {
    match &document.typed {
        StylelintConfigParseState::Parsed(_) => None,
        StylelintConfigParseState::Invalid(reason) => Some(reason),
    }
}

pub fn probe(document: &StylelintConfigDocument) -> Option<&StylelintEffectiveConfigProbe> {
    typed(document)?.probes.first()
}

pub fn rule_setting<'a>(
    document: &'a StylelintConfigDocument,
    rule_name: &str,
) -> Option<&'a serde_json::Value> {
    probe(document)?.rules.get(rule_name)
}
