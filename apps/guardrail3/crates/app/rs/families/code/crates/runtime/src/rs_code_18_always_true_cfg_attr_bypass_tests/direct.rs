use super::super::check_source;
use guardrail3_app_rs_family_code_assertions::rs_code_18_always_true_cfg_attr_bypass::{
    RuleFinding, assert_findings,
};

#[test]
fn errors_on_exhaustive_unix_windows_cfg_attr_allow() {
    let content = r#"
#[cfg_attr(any(unix, windows), allow(clippy::unwrap_used))]
fn foo() {}
"#;
    let results = check_source("src/foo.rs", content, false);

    assert_findings(
        &results,
        &[RuleFinding {
            severity: guardrail3_domain_report::Severity::Error,
            title: "always-true cfg_attr bypass",
            message: "`#[cfg_attr(..., allow(clippy::unwrap_used))]` is effectively unconditional. Use a direct `#[allow]` with an explicit reason instead.",
            file: Some("src/foo.rs"),
            line: Some(2),
            inventory: false,
        }],
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
        &[RuleFinding {
            severity: guardrail3_domain_report::Severity::Error,
            title: "always-true cfg_attr bypass",
            message: "`#[cfg_attr(..., allow(clippy::expect_used))]` is effectively unconditional. Use a direct `#[allow]` with an explicit reason instead.",
            file: Some("src/foo.rs"),
            line: Some(2),
            inventory: false,
        }],
    );
}

#[test]
fn errors_on_trait_item_with_always_true_cfg_attr_allow() {
    let content = "trait Api {\n    #[cfg_attr(all(), allow(dead_code))]\n    fn run();\n}\n";
    let results = check_source("src/lib.rs", content, false);

    assert_findings(
        &results,
        &[RuleFinding {
            severity: guardrail3_domain_report::Severity::Error,
            title: "always-true cfg_attr bypass",
            message: "`#[cfg_attr(..., allow(dead_code))]` is effectively unconditional. Use a direct `#[allow]` with an explicit reason instead.",
            file: Some("src/lib.rs"),
            line: Some(2),
            inventory: false,
        }],
    );
}
