use g3rs_code_config_checks_assertions::common::require_single_result;
use g3rs_code_config_checks_assertions::rs_code_config_07_exception_comment_inventory::assert_inventory_warn;
use g3rs_code_config_checks_types::{G3RsCodeConfigChecksInput, G3RsCodeExceptionCommentFact};

#[test]
fn emits_warn_for_each_exception_comment() {
    let input = G3RsCodeConfigChecksInput {
        exception_comments: vec![G3RsCodeExceptionCommentFact {
            rel_path: "deny.toml".to_owned(),
            line: 4,
            line_text: "# EXCEPTION: temporary".to_owned(),
        }],
        unsafe_code_lints: Vec::new(),
    };

    let results = crate::run::check(&input);
    let result = require_single_result(&results);
    assert_inventory_warn(result, "deny.toml", 4, "# EXCEPTION: temporary");
}
