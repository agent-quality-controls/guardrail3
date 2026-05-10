#![allow(
    clippy::expect_used,
    clippy::missing_panics_doc,
    reason = "assertion helpers are reusable panic-based proof sites for test harnesses"
)]

use eslint_config_parser_runtime::types::{
    EslintConfigFileKind, EslintConfigSnapshot, EslintProbeKind, EslintReportUnusedSetting,
    EslintRuleSeverity,
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

/// Returns the probe entry whose kind matches `probe_kind`, panicking when the snapshot has no such probe.
fn find_probe(
    snapshot: &EslintConfigSnapshot,
    probe_kind: EslintProbeKind,
) -> &eslint_config_parser_runtime::types::EslintEffectiveConfigProbe {
    snapshot
        .probes
        .iter()
        .find(|probe| probe.probe == probe_kind)
        .expect("probe should exist")
}

pub fn assert_project_service(
    snapshot: &EslintConfigSnapshot,
    probe_kind: EslintProbeKind,
    expected: Option<bool>,
) {
    assert_eq!(
        find_probe(snapshot, probe_kind).project_service,
        expected,
        "project_service mismatch"
    );
}

pub fn assert_probe_ignored(snapshot: &EslintConfigSnapshot, rel_path: &str, expected: bool) {
    let probe = snapshot
        .probes
        .iter()
        .find(|probe| probe.rel_path == rel_path)
        .expect("probe should exist");
    assert_eq!(probe.ignored, expected, "probe ignored state mismatch");
}

pub fn assert_plugins(
    snapshot: &EslintConfigSnapshot,
    probe_kind: EslintProbeKind,
    expected: &[&str],
) {
    assert_eq!(
        find_probe(snapshot, probe_kind).plugins,
        expected
            .iter()
            .map(|plugin| (*plugin).to_owned())
            .collect::<Vec<_>>(),
        "plugins mismatch"
    );
}

pub fn assert_plugin_meta_name(
    snapshot: &EslintConfigSnapshot,
    probe_kind: EslintProbeKind,
    namespace: &str,
    expected: &str,
) {
    assert_eq!(
        find_probe(snapshot, probe_kind)
            .plugin_meta_names
            .get(namespace)
            .map(String::as_str),
        Some(expected),
        "plugin metadata name mismatch"
    );
}

pub fn assert_plugin_package_name(
    snapshot: &EslintConfigSnapshot,
    probe_kind: EslintProbeKind,
    namespace: &str,
    expected: &str,
) {
    assert!(
        find_probe(snapshot, probe_kind)
            .plugin_package_names
            .get(namespace)
            .is_some_and(|package_names| package_names.iter().any(|name| name == expected)),
        "plugin package name mismatch"
    );
}

pub fn assert_no_inline_config(
    snapshot: &EslintConfigSnapshot,
    probe_kind: EslintProbeKind,
    expected: Option<bool>,
) {
    assert_eq!(
        find_probe(snapshot, probe_kind).linter_options_no_inline_config,
        expected,
        "noInlineConfig mismatch"
    );
}

pub fn assert_report_unused_disable_directives(
    snapshot: &EslintConfigSnapshot,
    probe_kind: EslintProbeKind,
    expected: Option<EslintReportUnusedSetting>,
) {
    assert_eq!(
        find_probe(snapshot, probe_kind).linter_options_report_unused_disable_directives,
        expected,
        "reportUnusedDisableDirectives mismatch"
    );
}

pub fn assert_report_unused_inline_configs(
    snapshot: &EslintConfigSnapshot,
    probe_kind: EslintProbeKind,
    expected: Option<EslintReportUnusedSetting>,
) {
    assert_eq!(
        find_probe(snapshot, probe_kind).linter_options_report_unused_inline_configs,
        expected,
        "reportUnusedInlineConfigs mismatch"
    );
}

pub fn assert_rule_severity(
    snapshot: &EslintConfigSnapshot,
    probe_kind: EslintProbeKind,
    rule_name: &str,
    expected: EslintRuleSeverity,
) {
    let rule = find_probe(snapshot, probe_kind)
        .rules
        .get(rule_name)
        .expect("rule should exist");
    assert_eq!(rule.severity, expected, "rule severity mismatch");
}

pub fn assert_rule_options_len(
    snapshot: &EslintConfigSnapshot,
    probe_kind: EslintProbeKind,
    rule_name: &str,
    expected: usize,
) {
    let rule = find_probe(snapshot, probe_kind)
        .rules
        .get(rule_name)
        .expect("rule should exist");
    assert_eq!(rule.options.len(), expected, "rule options length mismatch");
}

pub fn assert_parse_error(err: &impl std::fmt::Display) {
    let msg = err.to_string();
    assert!(
        msg.contains("eslint"),
        "error message should mention eslint evaluation, got: {msg}"
    );
}

pub fn assert_probe_kinds(snapshot: &EslintConfigSnapshot, expected: &[EslintProbeKind]) {
    let actual = snapshot
        .probes
        .iter()
        .map(|probe| probe.probe)
        .collect::<Vec<_>>();
    assert_eq!(actual, expected, "probe kinds mismatch");
}
