use std::collections::BTreeSet;

use super::helpers::copy_fixture;
use super::helpers::run_family;
use guardrail3_app_rs_family_code_assertions::lint_policy::rs_code_12_unsafe_code_lint::{
    assert_files, assert_populated_golden_fixture_inventories_workspace_forbid_lints,
};
use test_support::write_file;

#[test]
fn ignores_missing_or_non_workspace_unsafe_code_lints() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let rel = "packages/shared-types/Cargo.toml";
    let content = test_support::read_file(root, rel);

    write_file(
        root,
        rel,
        &format!("{content}\n[lints.rust]\nunsafe_code = \"deny\"\n"),
    );

    let results = run_family(root);

    assert_files(
        &results,
        BTreeSet::from([
            "apps/backend/Cargo.toml".to_owned(),
            "apps/devctl/Cargo.toml".to_owned(),
            "apps/worker/Cargo.toml".to_owned(),
        ]),
    );
    assert_populated_golden_fixture_inventories_workspace_forbid_lints(
        &results,
        "apps/backend/Cargo.toml",
        "apps/devctl/Cargo.toml",
        "apps/worker/Cargo.toml",
    );
}
