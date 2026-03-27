use guardrail3_domain_report::Severity;

use guardrail3_app_rs_family_code_assertions::rs_code_24_path_attr::{assert_normalized_len, findings};
use super::super::check_source;

#[test]
fn errors_on_path_attr_without_reason() {
    let content = "#[path = \"generated.rs\"]\nmod generated;";
    let binding = check_source("src/lib.rs", content, false);
    let results = findings(&binding);

    assert_normalized_len(&results, 1);
    assert_eq!(results[0].id, "RS-CODE-24");
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(results[0].title, "#[path] without reason");
    assert_eq!(
        results[0].message,
        "`#[path = \"generated.rs\"]` changes module resolution and requires `// reason:` on the same line."
    );
    assert_eq!(results[0].line, Some(1));
    assert_eq!(results[0].file.as_deref(), Some("src/lib.rs"));
    assert!(!results[0].inventory);
}

#[test]
fn warns_on_path_attr_with_reason() {
    let content = "#[path = \"generated.rs\"] // reason: generated facade shim\nmod generated;";
    let binding = check_source("src/lib.rs", content, false);
    let results = findings(&binding);

    assert_normalized_len(&results, 1);
    assert_eq!(results[0].id, "RS-CODE-24");
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].title, "#[path] usage");
    assert_eq!(
        results[0].message,
        "#[path = \"generated.rs\"] reason: generated facade shim"
    );
    assert_eq!(results[0].line, Some(1));
    assert_eq!(results[0].file.as_deref(), Some("src/lib.rs"));
    assert!(!results[0].inventory);
}

#[test]
fn errors_on_parent_escaping_path_attr() {
    let content = "#[path = \"../generated.rs\"]\nmod generated;";
    let binding = check_source("src/lib.rs", content, false);
    let results = findings(&binding);

    assert_normalized_len(&results, 1);
    assert_eq!(results[0].id, "RS-CODE-24");
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(results[0].title, "#[path] escapes parent directory");
    assert_eq!(
        results[0].message,
        "`#[path = \"../generated.rs\"]` escapes the standard module boundary."
    );
    assert_eq!(results[0].line, Some(1));
    assert_eq!(results[0].file.as_deref(), Some("src/lib.rs"));
    assert!(!results[0].inventory);
}

#[test]
fn skips_canonical_test_sidecar_path_wiring() {
    let content =
        "#[cfg(test)]\n#[path = \"rs_code_24_path_attr_tests/mod.rs\"]\nmod rs_code_24_path_attr_tests;";
    let binding = check_source("src/rs_code_24_path_attr.rs", content, false);
    let results = findings(&binding);

    assert_normalized_len(&results, 0);
}

#[test]
fn skips_documented_repo_standard_test_sidecar_path_wiring() {
    let content =
        "#[cfg(test)]\n#[path = \"rs_code_24_path_attr_tests/mod.rs\"]\nmod tests;";
    let binding = check_source("src/rs_code_24_path_attr.rs", content, false);
    let results = findings(&binding);

    assert_normalized_len(&results, 0);
}

#[test]
fn errors_on_near_miss_sidecar_path_wiring_without_cfg_test() {
    let content =
        "#[path = \"rs_code_24_path_attr_tests/mod.rs\"]\nmod rs_code_24_path_attr_tests;";
    let binding = check_source("src/lib.rs", content, false);
    let results = findings(&binding);

    assert_normalized_len(&results, 1);
    assert_eq!(results[0].id, "RS-CODE-24");
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(results[0].title, "#[path] without reason");
}

#[test]
fn errors_on_cfg_test_sidecar_path_for_another_rule_name() {
    let content =
        "#[cfg(test)]\n#[path = \"rs_code_99_other_rule_tests/mod.rs\"]\nmod rs_code_99_other_rule_tests;";
    let binding = check_source("src/lib.rs", content, false);
    let results = findings(&binding);

    assert_normalized_len(&results, 1);
    assert_eq!(results[0].id, "RS-CODE-24");
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(results[0].title, "#[path] without reason");
}

#[test]
fn errors_on_cfg_attr_parent_escaping_path_attr() {
    let content =
        "#[cfg_attr(unix, path = \"../generated.rs\")]\nmod generated;";
    let binding = check_source("src/lib.rs", content, false);
    let results = findings(&binding);

    assert_normalized_len(&results, 1);
    assert_eq!(results[0].id, "RS-CODE-24");
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(results[0].title, "#[path] escapes parent directory");
}
