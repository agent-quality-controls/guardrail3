use guardrail3_app_rs_family_deny_assertions::coverage::rs_deny_01_coverage as assertions;

use super::super::{copy_fixture, write_file};

#[test]
fn errors_when_an_allowed_deny_config_cannot_be_parsed() {
    let tmp = copy_fixture("../../../../../../../tests/fixtures/full_golden");
    write_file(tmp.path(), "deny.toml", "[sources");

    let results = super::super::run_family(tmp.path());
    assert!(!results.is_empty());
    assertions::assert_parse_error(&results, "deny.toml");
}
