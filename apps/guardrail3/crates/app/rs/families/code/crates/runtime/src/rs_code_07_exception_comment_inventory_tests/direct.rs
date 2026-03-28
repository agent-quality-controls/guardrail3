use guardrail3_domain_report::Severity;

use super::super::check_comment;
use guardrail3_app_rs_family_code_assertions::rs_code_07_exception_comment_inventory::{
    assert_normalized_len, findings,
};

#[test]
fn inventories_direct_exception_comment_input() {
    let line_text = "# EXCEPTION: temporary override";
    let raw_results = check_comment("Cargo.toml", 4, line_text);
    let results = findings(&raw_results);

    assert_normalized_len(&results, 1);
    assert_eq!(results[0].id, "RS-CODE-07");
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
    assert_eq!(results[0].file.as_deref(), Some("Cargo.toml"));
    assert_eq!(results[0].line, Some(4));
    assert_eq!(results[0].title, "EXCEPTION comment inventory");
    assert_eq!(
        results[0].message,
        format!("Config exception comment: {line_text}")
    );
}
