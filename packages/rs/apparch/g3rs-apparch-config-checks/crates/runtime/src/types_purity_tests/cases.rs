use g3rs_apparch_config_checks_assertions::types_purity as assertions;
use g3rs_apparch_types::{G3RsApparchDependencyKind, G3RsApparchRustPolicyState};

use super::helpers::{input, run_rule};

#[test]
fn impure_external_dependency_fires() {
    let results = run_rule(&input(
        "sqlx",
        G3RsApparchDependencyKind::BuildDependency,
        G3RsApparchRustPolicyState::Missing,
    ));

    assertions::assert_impure_dependency(&results, "types/core/Cargo.toml");
}

#[test]
fn built_in_allowed_dependency_emits_inventory() {
    let results = run_rule(&input(
        "serde",
        G3RsApparchDependencyKind::Dependency,
        G3RsApparchRustPolicyState::Missing,
    ));

    assertions::assert_clean_inventory(&results, "types/core/Cargo.toml");
}

#[test]
fn policy_allowlist_allows_extra_dependency() {
    let results = run_rule(&input(
        "reqwest",
        G3RsApparchDependencyKind::Dependency,
        G3RsApparchRustPolicyState::Parsed {
            rel_path: "guardrail3-rs.toml".to_owned(),
            profile: None,
            allowed_deps: vec!["reqwest".to_owned()],
            waivers: Vec::new(),
        },
    ));

    assertions::assert_clean_inventory(&results, "types/core/Cargo.toml");
}

#[test]
fn invalid_policy_fires_instead_of_dropping_to_empty_allowlist() {
    let results = run_rule(&input(
        "reqwest",
        G3RsApparchDependencyKind::Dependency,
        G3RsApparchRustPolicyState::ParseError {
            rel_path: "guardrail3-rs.toml".to_owned(),
            reason: "bad toml".to_owned(),
        },
    ));

    assertions::assert_policy_error_contains(&results, "types/core/Cargo.toml", "bad toml");
}

#[test]
fn unreadable_policy_fires_instead_of_dropping_to_empty_allowlist() {
    let results = run_rule(&input(
        "reqwest",
        G3RsApparchDependencyKind::Dependency,
        G3RsApparchRustPolicyState::Unreadable {
            rel_path: "guardrail3-rs.toml".to_owned(),
            reason: "permission denied".to_owned(),
        },
    ));

    assertions::assert_policy_error_contains(
        &results,
        "types/core/Cargo.toml",
        "permission denied",
    );
}
