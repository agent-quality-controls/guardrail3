use guardrail3_domain_report::Severity;

use guardrail3_app_rs_family_code_assertions::rs_code_18_always_true_cfg_attr_bypass::{assert_normalized_len, findings};
use super::super::check_source;

#[test]
fn errors_on_exhaustive_unix_windows_cfg_attr_allow() {
    let content = r#"
#[cfg_attr(any(unix, windows), allow(clippy::unwrap_used))]
fn foo() {}
"#;
    let raw_results = check_source("src/foo.rs", content, false);
    let results = findings(&raw_results);

    assert_normalized_len(&results, 1);
    assert_eq!(results[0].id, "RS-CODE-18");
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(results[0].file.as_deref(), Some("src/foo.rs"));
    assert_eq!(results[0].line, Some(2));
    assert_eq!(results[0].title, "always-true cfg_attr bypass");
    assert_eq!(
        results[0].message,
        "`#[cfg_attr(..., allow(clippy::unwrap_used))]` is effectively unconditional. Use a direct `#[allow]` with an explicit reason instead."
    );
}

#[test]
fn errors_on_empty_all_cfg_attr_allow() {
    let content = r#"
#[cfg_attr(all(), allow(clippy::expect_used))]
fn foo() {}
"#;
    let raw_results = check_source("src/foo.rs", content, false);
    let results = findings(&raw_results);

    assert_normalized_len(&results, 1);
    assert_eq!(results[0].id, "RS-CODE-18");
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(results[0].file.as_deref(), Some("src/foo.rs"));
    assert_eq!(results[0].line, Some(2));
    assert_eq!(results[0].title, "always-true cfg_attr bypass");
    assert_eq!(
        results[0].message,
        "`#[cfg_attr(..., allow(clippy::expect_used))]` is effectively unconditional. Use a direct `#[allow]` with an explicit reason instead."
    );
}

#[test]
fn errors_on_trait_item_with_always_true_cfg_attr_allow() {
    let content = "trait Api {\n    #[cfg_attr(all(), allow(dead_code))]\n    fn run();\n}\n";
    let raw_results = check_source("src/lib.rs", content, false);
    let results = findings(&raw_results);

    assert_normalized_len(&results, 1);
    assert_eq!(results[0].id, "RS-CODE-18");
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(results[0].file.as_deref(), Some("src/lib.rs"));
    assert_eq!(results[0].line, Some(2));
    assert_eq!(results[0].title, "always-true cfg_attr bypass");
    assert_eq!(
        results[0].message,
        "`#[cfg_attr(..., allow(dead_code))]` is effectively unconditional. Use a direct `#[allow]` with an explicit reason instead."
    );
}
