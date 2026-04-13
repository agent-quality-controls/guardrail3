use g3rs_code_config_checks_assertions::rs_code_config_07_exception_comment_inventory::assert_inventory_warn;

use super::helpers::{exception_comment, run_check};

#[test]
fn emits_warn_for_each_exception_comment() {
    let results = run_check(vec![
        exception_comment("deny.toml", 4, "# EXCEPTION: temporary"),
        exception_comment("Cargo.toml", 8, "// EXCEPTION: another"),
    ]);

    assert_eq!(results.len(), 2, "{results:#?}");
    assert_inventory_warn(&results[0], "deny.toml", 4, "# EXCEPTION: temporary");
    assert_inventory_warn(&results[1], "Cargo.toml", 8, "// EXCEPTION: another");
}
