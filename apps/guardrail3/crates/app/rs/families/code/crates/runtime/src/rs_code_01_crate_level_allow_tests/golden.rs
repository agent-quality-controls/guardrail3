use guardrail3_app_rs_family_code_assertions::rs_code_01_crate_level_allow::assert_no_hits;
use super::super::copy_fixture;
use super::super::run_family;
use test_support::write_file;

#[test]
fn populated_golden_fixture_has_no_crate_or_module_allow_hits() {
    let fixture = copy_fixture();
    let root = fixture.path();

    for rel in [
        "apps/backend/crates/app/queries/src/lib.rs",
        "apps/backend/crates/adapters/outbound/queue/src/lib.rs",
        "apps/backend/crates/ports/inbound/api/src/lib.rs",
        "apps/backend/tests/allow_inventory_tests.rs",
        "apps/worker/crates/adapters/outbound/sqs/src/lib.rs",
        "apps/devctl/tests/module_allow_tests.rs",
        "apps/worker/tests/crate_allow_tests.rs",
    ] {
        write_file(root, rel, "pub fn helper() {}\n");
    }

    let results = run_family(root);

    assert_no_hits(&results);
}
