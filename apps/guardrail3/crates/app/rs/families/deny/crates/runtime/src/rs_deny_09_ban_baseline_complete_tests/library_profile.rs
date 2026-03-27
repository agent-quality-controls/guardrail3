use guardrail3_domain_report::Severity;

use super::super::ConfigDenyInput;
use super::super::check;
use super::super::{build_fixture_deny_toml, config_facts_with_profile, remove_deny_ban};

#[test]
fn emits_no_result_for_generated_library_ban_baseline() {
    let results = super::super::run_check_with_profile(&build_fixture_deny_toml("library"), "library");

    assert!(
        results.is_empty(),
        "expected canonical library deny baseline to pass: {results:#?}"
    );
}

#[test]
fn library_profile_requires_library_io_bans() {
    let config = config_facts_with_profile(
        &remove_deny_ban(
            &remove_deny_ban(&build_fixture_deny_toml("library"), "axum"),
            "tokio",
        ),
        "library",
    );
    let input = ConfigDenyInput { config: &config };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 2);
    assert!(results.iter().any(|result| {
        result.id == "RS-DENY-09"
            && result.severity == Severity::Error
            && result.title == "missing canonical ban"
            && result.message == "`deny.toml` is missing deny ban `axum`."
            && result.file.as_deref() == Some("deny.toml")
    }));
    assert!(results.iter().any(|result| {
        result.id == "RS-DENY-09"
            && result.severity == Severity::Error
            && result.title == "missing canonical ban"
            && result.message == "`deny.toml` is missing deny ban `tokio`."
            && result.file.as_deref() == Some("deny.toml")
    }));
}
