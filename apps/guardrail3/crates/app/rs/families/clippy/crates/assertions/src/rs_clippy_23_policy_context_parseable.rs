use guardrail3_domain_report::{CheckResult, Severity};

const ID: &str = "RS-CLIPPY-23";

pub fn assert_inventory(results: &[CheckResult]) {
    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id(), ID);
    assert!(result.inventory());
    assert_eq!(result.severity(), Severity::Info);
    assert_eq!(result.title(), "clippy policy context parseable");
    assert_eq!(
        result.message(),
        "Active `guardrail3.toml` parsed successfully for clippy policy context."
    );
    assert_eq!(result.file(), Some("guardrail3.toml"));
}

pub fn assert_guardrail_parse_error(results: &[CheckResult], expected_fragment: &str) {
    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id(), ID);
    assert_eq!(result.severity(), Severity::Error);
    assert_eq!(result.title(), "clippy policy context is not parseable");
    assert!(
        result.message().contains(expected_fragment),
        "expected message to contain `{expected_fragment}`, got {:?}",
        result.message()
    );
    assert_eq!(result.file(), Some("guardrail3.toml"));
    assert!(!result.inventory());
}

pub fn assert_guardrail_content_missing(results: &[CheckResult]) {
    assert_guardrail_parse_error(results, "guardrail3.toml content missing from ProjectTree");
}
