use super::super::check_source;
use guardrail3_app_rs_family_code_assertions::api_shape::rs_code_33_public_weak_error_forms::{
    RuleFinding, assert_findings,
};

#[test]
fn errors_on_string_and_str_public_result_errors() {
    let results = check_source(
        "src/lib.rs",
        "pub fn parse() -> Result<(), String> { Ok(()) }\npub fn label() -> Result<(), &str> { Ok(()) }",
        false,
    );

    assert_findings(
        &results,
        &[
            RuleFinding::new(
                guardrail3_domain_report::Severity::Error,
                "weak public error form",
                "Public function `parse` returns `Result<_, String>`. Use a typed public error instead.",
                Some("src/lib.rs"),
                Some(1),
                false,
            ),
            RuleFinding::new(
                guardrail3_domain_report::Severity::Error,
                "weak public error form",
                "Public function `label` returns `Result<_, &str>`. Use a typed public error instead.",
                Some("src/lib.rs"),
                Some(2),
                false,
            ),
        ],
    );
}

#[test]
fn errors_on_anyhow_and_box_dyn_error_public_results() {
    let results = check_source(
        "src/lib.rs",
        "pub fn parse() -> Result<(), anyhow::Error> { Ok(()) }\npub fn boxed() -> Result<(), Box<dyn std::error::Error>> { Ok(()) }",
        false,
    );

    assert_findings(
        &results,
        &[
            RuleFinding::new(
                guardrail3_domain_report::Severity::Error,
                "weak public error form",
                "Public function `parse` returns `Result<_, anyhow::Error>`. Use a typed public error instead.",
                Some("src/lib.rs"),
                Some(1),
                false,
            ),
            RuleFinding::new(
                guardrail3_domain_report::Severity::Error,
                "weak public error form",
                "Public function `boxed` returns `Result<_, Box<dyn Error>>`. Use a typed public error instead.",
                Some("src/lib.rs"),
                Some(2),
                false,
            ),
        ],
    );
}
