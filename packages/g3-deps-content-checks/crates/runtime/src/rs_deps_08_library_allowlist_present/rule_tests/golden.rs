use super::helpers::run_check;

#[test]
fn inventories_allowlist_when_present() {
    let results = run_check("[profile]\nname = \"library\"\n[rust.packages]\nallowed_deps = [\"serde\"]\n");

    assert!(results.iter().any(|result| {
        result.id() == "RS-DEPS-08"
            && result.inventory()
            && result.message().contains("has an `allowed_deps` policy")
    }));
}

#[test]
fn warns_when_allowlist_missing() {
    let results = run_check("[profile]\nname = \"library\"\n");

    assert!(results.iter().any(|result| {
        result.id() == "RS-DEPS-08"
            && matches!(result.severity(), guardrail3_check_types::G3Severity::Warn)
            && result.message().contains("has no `allowed_deps` policy")
    }));
}
