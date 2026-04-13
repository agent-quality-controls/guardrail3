use g3rs_code_config_checks_assertions::rs_code_config_07_exception_comment_inventory::assert_inventory_warn;

use super::helpers::{run_check, text_file};

#[test]
fn emits_warn_for_each_exception_comment() {
    let results = run_check(vec![
        text_file("deny.toml", "\n\n\n# EXCEPTION: temporary\n"),
        text_file("Cargo.toml", "\n\n\n\n\n\n\n// EXCEPTION: another\n"),
    ]);

    assert_eq!(results.len(), 2, "{results:#?}");
    assert_inventory_warn(&results[0], "deny.toml", 4, "# EXCEPTION: temporary");
    assert_inventory_warn(&results[1], "Cargo.toml", 8, "// EXCEPTION: another");
}
