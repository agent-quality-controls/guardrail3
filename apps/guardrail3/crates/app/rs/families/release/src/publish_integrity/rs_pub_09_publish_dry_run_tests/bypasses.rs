use guardrail3_outbound_traits::CommandRunResult;

use super::super::check;
use super::super::{copy_fixture, crate_facts, crate_input, run_family, write_file};

#[test]
fn errors_when_publishable_crate_has_no_dry_run_result() {
    let facts = crate_facts("x");
    let input = crate_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    guardrail3_app_rs_family_release_assertions::rs_pub_09_publish_dry_run::assert_missing(
        &results,
        "crates/example/Cargo.toml",
    );
}

#[test]
fn errors_on_direct_publish_dry_run_failure_and_truncates_stderr() {
    let mut facts = crate_facts("x");
    facts.dry_run = Some(CommandRunResult::new(
        false,
        "line1\nline2\nline3\nline4".to_owned(),
    ));
    let input = crate_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    guardrail3_app_rs_family_release_assertions::rs_pub_09_publish_dry_run::assert_failed(
        &results,
        "crates/example/Cargo.toml",
        "x: publish dry-run failed",
        "line1; line2; line3",
    );
    guardrail3_app_rs_family_release_assertions::rs_pub_09_publish_dry_run::assert_no_message_contains(
        &results,
        "line4",
    );
}

#[test]
fn stays_out_of_scope_for_non_publishable_crate() {
    let mut facts = crate_facts("x");
    facts.publishable = false;
    facts.dry_run = Some(CommandRunResult::new(false, "boom".to_owned()));
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

    guardrail3_app_rs_family_release_assertions::rs_pub_09_publish_dry_run::assert_failed(
        &results,
        "packages/shared-types/Cargo.toml",
        "shared-types: publish dry-run failed",
        "publish --dry-run",
    );
}

#[test]
fn family_does_not_emit_publish_dry_run_results_when_not_thorough() {
    let tmp = copy_fixture();
    let results = run_family(tmp.path(), false);

    guardrail3_app_rs_family_release_assertions::rs_pub_09_publish_dry_run::assert_quiet(&results);
}
