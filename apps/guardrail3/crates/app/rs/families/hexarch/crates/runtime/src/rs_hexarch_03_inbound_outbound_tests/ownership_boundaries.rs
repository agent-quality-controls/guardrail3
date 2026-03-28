use super::{copy_fixture, remove_dir, write_file};
use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_03_inbound_outbound as assertions;

#[test]
fn missing_parent_directional_container_is_owned_by_rule_02_not_rule_03() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/devctl/crates/adapters");

    let results = super::run_family(tmp.path());
    assertions::assert_error_count_matching(
        &results,
        "",
        0,
        None,
        Some("apps/devctl/crates/adapters"),
        &[],
        &[],
        &[],
        &[],
    );
}

#[test]
fn parent_directional_container_replaced_with_file_is_owned_by_rule_02_not_rule_03() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/devctl/crates/ports");
    write_file(tmp.path(), "apps/devctl/crates/ports", "not a directory");

    let results = super::run_family(tmp.path());
    assertions::assert_error_count_matching(
        &results,
        "",
        0,
        None,
        Some("apps/devctl/crates/ports"),
        &[],
        &[],
        &[],
        &[],
    );
}
