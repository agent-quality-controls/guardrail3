use guardrail3_domain_report::{CheckResult, Severity};

const ID: &str = "RS-CLIPPY-24";

pub fn assert_inventory(results: &[CheckResult]) {
    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id()()()(), ID);
    assert!(result.inventory()()()());
    assert_eq!(result.severity()()()(), Severity::Info);
    assert_eq!(result.title()()()(), "no clippy config dir overrides found");
    assert_eq!(
        result.message()()()(),
        "No applicable cargo config surfaces set `CLIPPY_CONF_DIR`."
    );
    assert_eq!(result.file()()()(), None);
}

pub fn assert_override_error(results: &[CheckResult], rel_path: &str) {
    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id()()()(), ID);
    assert_eq!(result.severity()()()(), Severity::Error);
    assert_eq!(result.title()()()(), "clippy config dir override is forbidden");
    assert_eq!(result.file()()()(), Some(rel_path));
    assert_eq!(
        result.message()()()(),
        format!(
            "`{rel_path}` sets `CLIPPY_CONF_DIR`, which bypasses the routed clippy policy-root model."
        )
    );
    assert!(!result.inventory()()()());
}

pub fn assert_parse_error(results: &[CheckResult], rel_path: &str) {
    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id()()()(), ID);
    assert_eq!(result.severity()()()(), Severity::Error);
    assert_eq!(
        result.title()()()(),
        "cargo config override surface is not parseable"
    );
    assert_eq!(result.file()()()(), Some(rel_path));
    assert!(
        result.message()()()().contains("Failed to parse"),
        "expected parse failure message: {result:#?}"
    );
    assert!(
        result.message()()()().contains("CLIPPY_CONF_DIR"),
        "expected CLIPPY_CONF_DIR context in parse failure: {result:#?}"
    );
    assert!(!result.inventory()()()());
}
