use super::super::check_source;
use guardrail3_app_rs_family_code_assertions::rs_code_24_path_attr::{
    RuleFinding, assert_findings,
};

#[test]
fn errors_on_cfg_attr_parent_escaping_double_dot_filename_segment() {
    let content = "#[cfg_attr(test, path = \"generated../mod.rs\")]\nmod generated;";
    let results = check_source("src/lib.rs", content, false);

    assert_findings(
        &results,
        &[RuleFinding {
            severity: guardrail3_domain_report::Severity::Error,
            title: "#[path] without reason",
            message: "`#[path = \"generated../mod.rs\"]` changes module resolution and requires `// reason:` on the same line.",
            file: Some("src/lib.rs"),
            line: Some(1),
            inventory: false,
        }],
    );
}

#[test]
fn errors_on_cfg_attr_with_forged_reason_spelling() {
    let content =
        "#[cfg_attr(test, path = \"generated.rs\")] // REASON: generated seam\nmod generated;";
    let results = check_source("src/lib.rs", content, false);

    assert_findings(
        &results,
        &[RuleFinding {
            severity: guardrail3_domain_report::Severity::Error,
            title: "#[path] without reason",
            message: "`#[path = \"generated.rs\"]` changes module resolution and requires `// reason:` on the same line.",
            file: Some("src/lib.rs"),
            line: Some(1),
            inventory: false,
        }],
    );
}
