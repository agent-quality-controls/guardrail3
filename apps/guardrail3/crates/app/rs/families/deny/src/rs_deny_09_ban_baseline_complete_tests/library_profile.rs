use guardrail3_domain_report::Severity;

use super::super::super::inputs::ConfigDenyInput;
use super::super::super::test_support::{
    canonical_deny_toml_library, config_facts_with_profile, remove_deny_ban,
};
use super::super::check;

#[test]
fn emits_no_result_for_generated_library_ban_baseline() {
    let config = config_facts_with_profile(&canonical_deny_toml_library(), "library");
    let input = ConfigDenyInput { config: &config };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(
        results.is_empty(),
        "expected canonical library deny baseline to pass: {results:#?}"
    );
}

#[test]
fn library_profile_requires_library_io_bans() {
    let config = config_facts_with_profile(
        &remove_deny_ban(
            &remove_deny_ban(&canonical_deny_toml_library(), "axum"),
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
