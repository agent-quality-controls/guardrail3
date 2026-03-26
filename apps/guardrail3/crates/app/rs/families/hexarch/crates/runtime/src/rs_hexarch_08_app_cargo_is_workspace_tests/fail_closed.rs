use std::collections::BTreeSet;

use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_08_app_cargo_is_workspace as assertions;
use super::{copy_fixture, write_file};

#[test]
fn parse_error_hits_every_mutated_app() {
    let tmp = copy_fixture();
    for app in ["devctl", "backend", "worker"] {
        write_file(tmp.path(), &format!("apps/{app}/Cargo.toml"), "[workspace");
    }

    let results = super::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "");
    let actual_files = errors
        .iter()
        .filter_map(|error| error.file.clone())
        .collect::<BTreeSet<_>>();
    let expected_files = [
        "apps/devctl/Cargo.toml",
        "apps/backend/Cargo.toml",
        "apps/worker/Cargo.toml",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect::<BTreeSet<_>>();

    assert_eq!(
        actual_files, expected_files,
        "unexpected hit set: {errors:#?}"
    );
    for error in &errors {
        assert!(error.title.contains("invalid workspace config"));
    }
}

#[test]
fn non_string_workspace_member_is_invalid_app_cargo() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/devctl/Cargo.toml",
        r#"[workspace]
members = [
    "crates/domain/types",
    42,
]
resolver = "2"
"#,
    );

    let results = super::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "");

    assert_eq!(
        errors.len(),
        1,
        "expected one invalid workspace members error: {errors:#?}"
    );
    assert_eq!(errors[0].file.as_deref(), Some("apps/devctl/Cargo.toml"));
    assert!(
        errors[0].title.contains("invalid workspace config"),
        "expected invalid workspace ownership: {errors:#?}"
    );
    assert!(
        errors[0]
            .message
            .contains("[workspace].members[1] must be a string"),
        "expected exact semantic parse message: {errors:#?}"
    );
}
