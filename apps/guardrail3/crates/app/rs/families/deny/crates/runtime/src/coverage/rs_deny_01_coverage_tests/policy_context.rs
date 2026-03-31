use guardrail3_app_rs_family_deny_assertions::rs_deny_01_coverage as coverage_assertions;

use super::super::{build_fixture_deny_toml, run_family};
use test_support::{copy_fixture, remove_deny_ban, write_file};

#[test]
fn malformed_guardrail_policy_emits_an_explicit_error_and_skips_profile_sensitive_rules() {
    let tmp = copy_fixture("../../../../../../../tests/fixtures/full_golden");
    write_file(tmp.path(), "guardrail3.toml", "[profile =");
    write_file(
        tmp.path(),
        "deny.toml",
        &remove_deny_ban(&build_fixture_deny_toml("library"), "axum"),
    );

    let results = run_family(tmp.path());

    coverage_assertions::assert_policy_context_parse_error(
        &results,
        "Failed to parse active `guardrail3.toml` used for deny profile selection",
    );
    coverage_assertions::assert_no_findings_for(&results, "RS-DENY-09");
    coverage_assertions::assert_no_findings_for(&results, "RS-DENY-25");
    coverage_assertions::assert_no_findings_for(&results, "RS-DENY-30");
}

#[test]
fn unknown_guardrail_profile_emits_an_explicit_error_and_skips_profile_sensitive_rules() {
    let tmp = copy_fixture("../../../../../../../tests/fixtures/full_golden");
    write_file(tmp.path(), "guardrail3.toml", "[profile]\nname = \"cli\"\n");
    write_file(
        tmp.path(),
        "deny.toml",
        &remove_deny_ban(&build_fixture_deny_toml("library"), "axum"),
    );

    let results = run_family(tmp.path());

    coverage_assertions::assert_policy_context_parse_error(
        &results,
        "`profile.name` must be `service` or `library`",
    );
    coverage_assertions::assert_no_findings_for(&results, "RS-DENY-09");
    coverage_assertions::assert_no_findings_for(&results, "RS-DENY-25");
    coverage_assertions::assert_no_findings_for(&results, "RS-DENY-30");
}
