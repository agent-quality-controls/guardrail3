use guardrail3_domain_report::Severity;

#[test]
fn errors_when_bans_section_is_missing() {
    let results = super::super::run_check("[graph]\nall-features = true\n");

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-DENY-09");
    assert_eq!(result.severity, Severity::Error);
    assert_eq!(result.title, "[bans] section missing");
    assert_eq!(result.message, "`deny.toml` has no `[bans]` section.");
    assert_eq!(result.file.as_deref(), Some("deny.toml"));
}

#[test]
fn errors_when_bans_deny_is_missing() {
    let results = super::super::run_check("[bans]\nmultiple-versions = \"deny\"\n");

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-DENY-09");
    assert_eq!(result.severity, Severity::Error);
    assert_eq!(result.title, "[bans].deny missing");
    assert_eq!(result.message, "`deny.toml` must contain `[bans].deny`.");
    assert_eq!(result.file.as_deref(), Some("deny.toml"));
}
