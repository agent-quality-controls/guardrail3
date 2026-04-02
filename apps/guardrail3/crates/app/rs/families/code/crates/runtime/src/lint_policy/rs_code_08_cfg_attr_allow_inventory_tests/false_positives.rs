use super::super::copy_fixture;
use super::super::run_family;
use guardrail3_app_rs_family_code_assertions::lint_policy::rs_code_08_cfg_attr_allow_inventory::{
    assert_no_hits, assert_normalized_empty, findings,
};
use test_support::write_file;

#[test]
fn skips_always_true_and_non_cfg_attr_allow_surfaces() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let always_true_rel = "apps/backend/crates/app/queries/src/lib.rs";
    let plain_item_rel = "apps/devctl/crates/app/core/src/lib.rs";
    let crate_level_rel = "apps/worker/crates/app/processor/src/lib.rs";
    let grouped_conditional_rel = "apps/backend/crates/ports/outbound/repo/src/lib.rs";
    let nested_always_true_rel = "apps/backend/crates/app/commands/src/lib.rs";

    let always_true_content = test_support::read_file(root, always_true_rel);
    let plain_item_content = test_support::read_file(root, plain_item_rel);
    let crate_level_content = test_support::read_file(root, crate_level_rel);
    let grouped_conditional_content = test_support::read_file(root, grouped_conditional_rel);
    let nested_always_true_content = test_support::read_file(root, nested_always_true_rel);

    write_file(
        root,
        always_true_rel,
        &format!(
            "{always_true_content}\n#[cfg_attr(all(), allow(clippy::unwrap_used))]\nfn always_true_probe() {{}}\n"
        ),
    );
    write_file(
        root,
        plain_item_rel,
        &format!(
            "{plain_item_content}\n#[allow(clippy::unwrap_used)]\nfn plain_allow_probe() {{}}\n"
        ),
    );
    write_file(
        root,
        crate_level_rel,
        &format!("#![allow(clippy::expect_used)]\n{crate_level_content}\n"),
    );
    write_file(
        root,
        grouped_conditional_rel,
        &format!(
            "{grouped_conditional_content}\n#[cfg_attr(all(), allow(clippy::panic))]\nfn grouped_always_true_probe() {{}}\n"
        ),
    );
    write_file(
        root,
        nested_always_true_rel,
        &format!(
            "{nested_always_true_content}\n#[cfg_attr(all(), cfg_attr(all(), allow(clippy::expect_used)))]\nfn nested_always_true_probe() {{}}\n"
        ),
    );

    let results = run_family(root);
    let rs_code_08_results = findings(&results);

    assert_no_hits(&results);
    assert_normalized_empty(&rs_code_08_results);
}
