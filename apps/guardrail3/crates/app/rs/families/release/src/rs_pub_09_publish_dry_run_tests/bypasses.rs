use guardrail3_domain_report::Severity;
use guardrail3_outbound_traits::CommandRunResult;

use super::super::super::test_support::{
    copy_fixture, crate_facts, crate_input, errors_by_id, run_family, write_file,
};
use super::super::check;

#[test]
fn errors_when_publishable_crate_has_no_dry_run_result() {
    let facts = crate_facts("x");
    let input = crate_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-PUB-09");
    assert_eq!(results[0].severity, Severity::Error);
    assert!(!results[0].inventory);
    assert_eq!(
        results[0].file.as_deref(),
        Some("crates/example/Cargo.toml")
    );
    assert!(results[0].title.contains("publish dry-run missing"));
    assert!(results[0].message.contains("thorough mode"));
}

#[test]
fn errors_on_direct_publish_dry_run_failure_and_truncates_stderr() {
    let mut facts = crate_facts("x");
    facts.dry_run = Some(CommandRunResult {
        success: false,
        stderr: "line1\nline2\nline3\nline4".to_owned(),
    });
    let input = crate_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-PUB-09");
    assert_eq!(results[0].severity, Severity::Error);
    assert!(!results[0].inventory);
    assert_eq!(
        results[0].file.as_deref(),
        Some("crates/example/Cargo.toml")
    );
    assert_eq!(results[0].title, "x: publish dry-run failed");
    assert!(results[0].message.contains("line1; line2; line3"));
    assert!(!results[0].message.contains("line4"));
}

#[test]
fn stays_out_of_scope_for_non_publishable_crate() {
    let mut facts = crate_facts("x");
    facts.publishable = false;
    facts.dry_run = Some(CommandRunResult {
        success: false,
        stderr: "boom".to_owned(),
    });
    let input = crate_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(results.is_empty());
}

#[test]
fn errors_on_real_publish_dry_run_failure_from_broken_fixture_crate() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "packages/shared-types/src/lib.rs",
        "pub fn broken( {",
    );

    let results = run_family(tmp.path(), true);
    let failures = errors_by_id(&results, "RS-PUB-09");
    let failure = failures
        .iter()
        .find(|result| result.file.as_deref() == Some("packages/shared-types/Cargo.toml"))
        .expect("expected RS-PUB-09 failure for broken shared-types crate");

    assert_eq!(failure.severity, Severity::Error);
    assert_eq!(failure.title, "shared-types: publish dry-run failed");
    assert!(failure.message.contains("publish --dry-run"));
}

#[test]
fn family_does_not_emit_publish_dry_run_results_when_not_thorough() {
    let tmp = copy_fixture();
    let results = run_family(tmp.path(), false);

    assert!(
        results.iter().all(|result| result.id != "RS-PUB-09"),
        "unexpected RS-PUB-09 results in non-thorough mode: {results:#?}"
    );
}
