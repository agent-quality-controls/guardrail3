use super::super::copy_fixture;
use super::super::run_family;
use guardrail3_app_rs_family_code_assertions::lint_policy::rs_code_12_unsafe_code_lint::assert_populated_golden_fixture_inventories_workspace_forbid_lints;

#[test]
fn populated_golden_fixture_inventories_workspace_forbid_lints() {
    let fixture = copy_fixture();

    assert_populated_golden_fixture_inventories_workspace_forbid_lints(
        &run_family(fixture.path()),
        "apps/backend/Cargo.toml",
        "apps/devctl/Cargo.toml",
        "apps/worker/Cargo.toml",
    );
}
