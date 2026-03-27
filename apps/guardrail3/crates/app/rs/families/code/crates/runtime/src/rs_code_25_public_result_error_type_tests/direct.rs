use guardrail3_domain_report::Severity;

use guardrail3_app_rs_family_code_assertions::rs_code_25_public_result_error_type::{assert_normalized_len, findings};
use super::super::check_source;

#[test]
fn warns_on_public_result_string_in_library_profile() {
    let content = "pub fn parse() -> Result<(), String> { Ok(()) }";
    let binding = check_source("src/lib.rs", content, false);
    let results = findings(&binding);

    assert_normalized_len(&results, 1);
    assert_eq!(results[0].id, "RS-CODE-25");
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].title, "weak public error type");
    assert_eq!(
        results[0].message,
        "Public function `parse` returns `Result<_, String>`. Use a typed error instead."
    );
    assert_eq!(results[0].line, Some(1));
    assert_eq!(results[0].file.as_deref(), Some("src/lib.rs"));
    assert!(!results[0].inventory);
}

#[test]
fn warns_on_public_result_box_dyn_error_in_library_profile() {
    let content = "pub fn parse() -> Result<(), Box<dyn std::error::Error>> { Ok(()) }";
    let binding = check_source("src/lib.rs", content, false);
    let results = findings(&binding);

    assert_normalized_len(&results, 1);
    assert_eq!(results[0].id, "RS-CODE-25");
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].title, "weak public error type");
    assert_eq!(
        results[0].message,
        "Public function `parse` returns `Result<_, Box<dyn Error>>`. Use a typed error instead."
    );
    assert_eq!(results[0].line, Some(1));
    assert_eq!(results[0].file.as_deref(), Some("src/lib.rs"));
    assert!(!results[0].inventory);
}

#[test]
fn warns_on_public_trait_method_result_string_in_library_profile() {
    let content = "pub trait Service { fn parse(&self) -> Result<(), String>; }";
    let binding = check_source("src/lib.rs", content, false);
    let results = findings(&binding);

    assert_normalized_len(&results, 1);
    assert_eq!(results[0].id, "RS-CODE-25");
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].title, "weak public error type");
    assert_eq!(
        results[0].message,
        "Public function `Service::parse` returns `Result<_, String>`. Use a typed error instead."
    );
    assert_eq!(results[0].line, Some(1));
    assert_eq!(results[0].file.as_deref(), Some("src/lib.rs"));
    assert!(!results[0].inventory);
}
