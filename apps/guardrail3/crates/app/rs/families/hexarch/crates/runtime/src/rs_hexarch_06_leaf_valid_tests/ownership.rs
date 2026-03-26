use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_06_leaf_valid as assertions;
use crate::test_support::{copy_fixture, remove_dir, write_file};
const FIXTURE: crate::test_support::HexarchFixture = crate::test_support::HexarchFixture;

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

    let results = assertions::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-06");
    assert_eq!(errors.len(), 1, "{errors:#?}");
    assert_eq!(
        errors[0].file.as_deref(),
        Some("apps/backend/crates/adapters/inbound/mcp/crates/app/orphan_inner")
    );
    assert!(
        errors[0]
            .title
            .contains("crates/adapters/inbound/mcp/crates/app/orphan_inner"),
        "{errors:#?}"
    );
}

#[test]
fn destroying_outer_parent_does_not_create_nested_rule_06_hits() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/backend/crates/adapters/inbound");

    let results = assertions::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-06");
    assert!(
        errors
            .iter()
            .all(|error| !error.file.as_deref().unwrap_or("").contains(inner_hex())),
        "rule 06 should not materialize nested leaves under a destroyed outer parent: {errors:#?}"
    );
}
