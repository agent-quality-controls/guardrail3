use super::helpers::run_check;
use guardrail3_rs_toml_parser::RustProfile;

#[test]
fn inventories_allowlist_when_present() {
    let results = run_check(Some(RustProfile::Library), true);

    assert!(results.iter().any(|result| {
        result.id() == "RS-DEPS-CONFIG-04"
            && result.inventory()
            && result.message().contains("has an `allowed_deps` policy")
    }));
}

#[test]
fn warns_when_allowlist_missing() {
    let results = run_check(Some(RustProfile::Library), false);

    assert!(results.iter().any(|result| {
        result.id() == "RS-DEPS-CONFIG-04"
            && matches!(result.severity(), guardrail3_check_types::G3Severity::Warn)
            && result.message().contains("has no `allowed_deps` policy")
    }));
}
