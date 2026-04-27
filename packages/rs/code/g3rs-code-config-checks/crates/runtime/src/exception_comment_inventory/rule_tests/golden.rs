use g3rs_code_config_checks_assertions::exception_comment_inventory::rule::assert_two_inventory_warns;

use super::helpers::{exception_comment, run_check};

#[test]
fn emits_warn_for_each_exception_comment() {
    let results = run_check(vec![
        exception_comment("deny.toml", 4, "# EXCEPTION: temporary"),
        exception_comment("Cargo.toml", 8, "// EXCEPTION: another"),
    ]);

    assert_two_inventory_warns(
        &results,
        "deny.toml",
        4,
        "# EXCEPTION: temporary",
        "Cargo.toml",
        8,
        "// EXCEPTION: another",
    );
}
