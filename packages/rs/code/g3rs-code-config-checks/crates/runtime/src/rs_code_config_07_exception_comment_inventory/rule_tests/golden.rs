use g3rs_code_config_checks_assertions::rs_code_config_07_exception_comment_inventory::assert_inventory_warn;
use g3rs_code_config_checks_types::G3RsCodeExceptionCommentFact;

use super::helpers::run_check;

#[test]
fn emits_warn_for_each_exception_comment() {
    let results = run_check(vec![
        G3RsCodeExceptionCommentFact {
            rel_path: "deny.toml".to_owned(),
            line: 4,
            line_text: "# EXCEPTION: temporary".to_owned(),
        },
        G3RsCodeExceptionCommentFact {
            rel_path: "Cargo.toml".to_owned(),
            line: 8,
            line_text: "// EXCEPTION: another".to_owned(),
        },
    ]);

    assert_eq!(results.len(), 2, "{results:#?}");
    assert_inventory_warn(&results[0], "deny.toml", 4, "# EXCEPTION: temporary");
    assert_inventory_warn(&results[1], "Cargo.toml", 8, "// EXCEPTION: another");
}
