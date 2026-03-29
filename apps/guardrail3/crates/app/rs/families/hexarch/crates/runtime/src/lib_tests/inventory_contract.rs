use guardrail3_app_rs_family_hexarch_assertions::{
    HEXARCH_INVENTORY_RULE_IDS, PATCH_REPLACE_BYPASS_RULE_ID, assert_inventory_ids,
    assert_inventory_result,
};

use super::{copy_fixture, run_family, write_file};

#[test]
fn golden_fixture_emits_inventory_for_every_relevant_success_rule() {
    let tmp = copy_fixture();
    let results = run_family(tmp.path());
    assert!(!results.is_empty(), "{results:#?}");

    assert_inventory_ids(&results, HEXARCH_INVENTORY_RULE_IDS);
}

#[test]
fn patch_entry_outside_layered_tree_emits_success_inventory() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/backend/Cargo.toml",
        "[workspace]\nmembers = [\n    \"crates/domain/types\",\n    \"crates/domain/engine\",\n    \"crates/app/commands\",\n    \"crates/app/queries\",\n    \"crates/ports/inbound/api\",\n    \"crates/ports/outbound/repo\",\n    \"crates/ports/outbound/events\",\n    \"crates/adapters/inbound/rest\",\n    \"crates/adapters/outbound/postgres\",\n    \"crates/adapters/outbound/queue\",\n]\nresolver = \"2\"\n\n[patch.crates-io]\nshared-types = { path = \"../../packages/shared-types\" }\n",
    );

    let results = run_family(tmp.path());
    assert!(!results.is_empty(), "{results:#?}");
    assert_inventory_result(
        &results,
        PATCH_REPLACE_BYPASS_RULE_ID,
        "apps/backend/Cargo.toml",
        1,
        "outside the owned layered Rust tree",
    );
}
