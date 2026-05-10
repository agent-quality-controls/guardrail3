use eslint_config_parser_types::document::{
    EslintConfigDocument, EslintConfigParseState, EslintConfigSnapshot, EslintEffectiveConfigProbe,
    EslintProbeKind, EslintRuleSetting,
};

#[must_use]
pub const fn typed(document: &EslintConfigDocument) -> Option<&EslintConfigSnapshot> {
    match &document.typed {
        EslintConfigParseState::Parsed(snapshot) => Some(snapshot),
        EslintConfigParseState::Invalid(_) => None,
    }
}

#[must_use]
pub fn parse_error_reason(document: &EslintConfigDocument) -> Option<&str> {
    match &document.typed {
        EslintConfigParseState::Parsed(_) => None,
        EslintConfigParseState::Invalid(reason) => Some(reason),
    }
}

#[must_use]
pub fn probe(
    document: &EslintConfigDocument,
    probe_kind: EslintProbeKind,
) -> Option<&EslintEffectiveConfigProbe> {
    typed(document)?
        .probes
        .iter()
        .find(|probe| probe.probe == probe_kind)
}

#[must_use]
pub fn rule_setting<'a>(
    document: &'a EslintConfigDocument,
    probe_kind: EslintProbeKind,
    rule_name: &str,
) -> Option<&'a EslintRuleSetting> {
    probe(document, probe_kind)?.rules.get(rule_name)
}
