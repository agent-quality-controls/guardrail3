use super::super::check_source;
use guardrail3_app_rs_family_code_assertions::cfg_and_paths::rs_code_18_always_true_cfg_attr_bypass::{
    RuleFinding, assert_findings,
};

#[test]
fn errors_on_not_any_cfg_attr_allow() {
    let content = r#"
#[cfg_attr(not(any()), allow(clippy::unwrap_used))]
fn foo() {}
"#;
    let results = check_source("src/foo.rs", content, false);

    assert_findings(
        &results,
        &[RuleFinding::new(
            guardrail3_domain_report::Severity::Error,
            "always-true cfg_attr bypass",
            "`#[cfg_attr(..., allow(clippy::unwrap_used))]` is effectively unconditional. Use a direct `#[allow]` with an explicit reason instead.",
            Some("src/foo.rs"),
            Some(2),
            false,
        )],
    );
}

#[test]
fn errors_on_empty_all_cfg_attr_allow() {
    let content = r#"
#[cfg_attr(all(), allow(clippy::expect_used))]
fn foo() {}
"#;
    let results = check_source("src/foo.rs", content, false);

    assert_findings(
        &results,
        &[RuleFinding::new(
            guardrail3_domain_report::Severity::Error,
            "always-true cfg_attr bypass",
            "`#[cfg_attr(..., allow(clippy::expect_used))]` is effectively unconditional. Use a direct `#[allow]` with an explicit reason instead.",
            Some("src/foo.rs"),
            Some(2),
            false,
        )],
    );
}

#[test]
fn errors_on_trait_item_with_always_true_cfg_attr_allow() {
    let content = "trait Api {\n    #[cfg_attr(all(), allow(dead_code))]\n    fn run();\n}\n";
    let results = check_source("src/lib.rs", content, false);

    assert_findings(
        &results,
        &[RuleFinding::new(
            guardrail3_domain_report::Severity::Error,
            "always-true cfg_attr bypass",
            "`#[cfg_attr(..., allow(dead_code))]` is effectively unconditional. Use a direct `#[allow]` with an explicit reason instead.",
            Some("src/lib.rs"),
            Some(2),
            false,
        )],
    );
}
