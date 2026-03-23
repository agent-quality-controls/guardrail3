use crate::domain::report::Severity;
use crate::ports::outbound::CommandRunResult;

use super::super::super::test_support::{
    copy_fixture, crate_facts, crate_input, errors_by_id, run_family, write_file,
};
use super::super::check;

#[test]
fn stays_out_of_scope_without_dry_run_result() {
    let facts = crate_facts("x");
    let input = crate_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(results.is_empty());
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
    assert!(failure.message.contains("cargo publish --dry-run failed"));
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
