use super::super::check_comment;
use guardrail3_app_rs_family_code_assertions::lint_policy::rs_code_07_exception_comment_inventory::{
    RuleFinding, assert_findings,
};

#[test]
fn inventories_direct_exception_comment_input() {
    let line_text = "# EXCEPTION: temporary override";
    let results = check_comment("Cargo.toml", 4, line_text);

    assert_findings(
        &results,
        &[RuleFinding::new(
            guardrail3_domain_report::Severity::Info,
            "EXCEPTION comment inventory",
            &format!("Config exception comment: {line_text}"),
            Some("Cargo.toml"),
            Some(4),
            true,
        )],
    );
}
