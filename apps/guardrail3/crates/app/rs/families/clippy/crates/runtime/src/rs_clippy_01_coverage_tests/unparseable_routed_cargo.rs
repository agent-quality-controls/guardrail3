use guardrail3_domain_report::Severity;
use test_support::{build_fixture_clippy_toml, create_dir_all, create_temp_dir, write_file};

use super::super::run_for_tests;

#[test]
fn errors_when_a_routed_cargo_root_cannot_be_parsed() {
    let tmp = create_temp_dir("rs-clippy-01-unparseable-routed-cargo");
    create_dir_all(&tmp.path().join("apps/backend/crates/core"));
    write_file(
        tmp.path(),
        "apps/backend/Cargo.toml",
        "[workspace\nmembers = [\"crates/*\"]\n",
    );
    write_file(
        tmp.path(),
        "apps/backend/clippy.toml",
        &build_fixture_clippy_toml("service", false, true, "", ""),
    );
    write_file(
        tmp.path(),
        "apps/backend/crates/core/Cargo.toml",
        "[package]\nname = \"core\"\n",
    );

    let results = run_for_tests(tmp.path());
    let failures = results
        .iter()
        .filter(|result| {
            result.id == "RS-CLIPPY-01"
                && result.severity == Severity::Error
                && result.title == "Rust unit coverage could not be determined"
        })
        .collect::<Vec<_>>();

    assert_eq!(failures.len(), 1);
    let result = failures[0];
    assert_eq!(result.title, "Rust unit coverage could not be determined");
    assert_eq!(result.file.as_deref(), Some("apps/backend/Cargo.toml"));
    assert!(
        result
            .message
            .contains("routed Cargo root `apps/backend` could not be parsed"),
        "expected routed-root parse failure message: {result:#?}"
    );
    assert!(
        result
            .message
            .contains("while resolving clippy coverage and policy roots"),
        "expected coverage/root-resolution context: {result:#?}"
    );
    assert!(!result.inventory);
}
