use eslint_config_parser_runtime::types::{
    EslintConfigFileKind, EslintConfigSnapshot, EslintProbeKind, EslintRuleSeverity,
};

pub fn assert_selected_config(
    snapshot: &EslintConfigSnapshot,
    rel_path: &str,
    kind: EslintConfigFileKind,
) {
    assert_eq!(
        snapshot.selected_config.rel_path, rel_path,
        "selected config path mismatch"
    );
    assert_eq!(
        snapshot.selected_config.kind, kind,
        "selected config kind mismatch"
    );
}

pub fn assert_project_service(
    snapshot: &EslintConfigSnapshot,
    probe_kind: EslintProbeKind,
    expected: Option<bool>,
) {
    let probe = snapshot
        .probes
        .iter()
        .find(|probe| probe.probe == probe_kind)
        .expect("probe should exist");
    assert_eq!(probe.project_service, expected, "project_service mismatch");
}

pub fn assert_rule_severity(
    snapshot: &EslintConfigSnapshot,
    probe_kind: EslintProbeKind,
    rule_name: &str,
    expected: EslintRuleSeverity,
) {
    let probe = snapshot
        .probes
        .iter()
        .find(|probe| probe.probe == probe_kind)
        .expect("probe should exist");
    let rule = probe.rules.get(rule_name).expect("rule should exist");
    assert_eq!(rule.severity, expected, "rule severity mismatch");
}

pub fn assert_parse_error(err: &impl std::fmt::Display) {
    let msg = err.to_string();
    assert!(
        msg.contains("eslint"),
        "error message should mention eslint evaluation, got: {msg}"
    );
}
