use super::super::check_comment;
use guardrail3_app_rs_family_code_assertions::rs_code_07_exception_comment_inventory::{
    RuleFinding, assert_findings,
};

#[test]
fn inventories_direct_exception_comment_input() {
    let line_text = "# EXCEPTION: temporary override";
    let results = check_comment("Cargo.toml", 4, line_text);

    assert_findings(
        &results,
        &[RuleFinding {
            severity: guardrail3_domain_report::Severity::Info,
            title: "EXCEPTION comment inventory",
            message: &format!("Config exception comment: {line_text}"),
            file: Some("Cargo.toml"),
            line: Some(4),
            inventory: true,
        }],
    );
}
