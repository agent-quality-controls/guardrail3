use super::super::check_source;
use guardrail3_app_rs_family_code_assertions::rs_code_25_public_result_error_type::{
    RuleFinding, assert_findings,
};

#[test]
fn warns_on_public_result_string_in_library_profile() {
    let content = "pub fn parse() -> Result<(), String> { Ok(()) }";
    let results = check_source("src/lib.rs", content, false);

    assert_findings(
        &results,
        &[RuleFinding {
            severity: guardrail3_domain_report::Severity::Warn,
            title: "weak public error type",
            message: "Public function `parse` returns `Result<_, String>`. Use a typed error instead.",
            file: Some("src/lib.rs"),
            line: Some(1),
            inventory: false,
        }],
    );
}

#[test]
fn warns_on_public_result_box_dyn_error_in_library_profile() {
    let content = "pub fn parse() -> Result<(), Box<dyn std::error::Error>> { Ok(()) }";
    let results = check_source("src/lib.rs", content, false);

    assert_findings(
        &results,
        &[RuleFinding {
            severity: guardrail3_domain_report::Severity::Warn,
            title: "weak public error type",
            message: "Public function `parse` returns `Result<_, Box<dyn Error>>`. Use a typed error instead.",
            file: Some("src/lib.rs"),
            line: Some(1),
            inventory: false,
        }],
    );
}

#[test]
fn warns_on_public_trait_method_result_string_in_library_profile() {
    let content = "pub trait Service { fn parse(&self) -> Result<(), String>; }";
    let results = check_source("src/lib.rs", content, false);

    assert_findings(
        &results,
        &[RuleFinding {
            severity: guardrail3_domain_report::Severity::Warn,
            title: "weak public error type",
            message: "Public function `Service::parse` returns `Result<_, String>`. Use a typed error instead.",
            file: Some("src/lib.rs"),
            line: Some(1),
            inventory: false,
        }],
    );
}
