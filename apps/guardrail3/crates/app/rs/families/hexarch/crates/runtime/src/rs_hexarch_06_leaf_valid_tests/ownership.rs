use super::{copy_fixture, remove_dir, write_file};
use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_06_leaf_valid as assertions;
const FIXTURE: super::HexarchFixture = super::HexarchFixture;

fn inner_hex() -> &'static str {
    FIXTURE.inner_hex_root()
}

#[test]
fn inner_hex_invalid_leaf_hits_only_inner_hex_and_leaves_outer_apps_clean() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        &format!("{}/app/orphan_inner/src/lib.rs", inner_hex()),
        "pub fn orphan_inner() {}\n",
    );

    let results = super::run_family(tmp.path());
    assertions::assert_expected_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            file: Some("apps/backend/crates/adapters/inbound/mcp/crates/app/orphan_inner"),
            file_contains: None,
            title_contains: Some(&["crates/adapters/inbound/mcp/crates/app/orphan_inner"]),
            message_contains: None,
        }],
    );
}

#[test]
fn destroying_outer_parent_does_not_create_nested_rule_06_hits() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/backend/crates/adapters/inbound");

    let results = super::run_family(tmp.path());
    assertions::assert_no_error_file_contains(&results, inner_hex());
}
