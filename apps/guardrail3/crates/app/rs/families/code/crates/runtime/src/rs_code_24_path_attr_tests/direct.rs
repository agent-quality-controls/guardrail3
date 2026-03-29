use super::super::check_source;
use guardrail3_app_rs_family_code_assertions::rs_code_24_path_attr::{
    RuleFinding, assert_findings, assert_no_hits,
};

#[test]
fn errors_on_path_attr_without_reason() {
    let results = check_source(
        "src/lib.rs",
        "#[path = \"generated.rs\"]\nmod generated;",
        false,
    );

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

#[test]
fn warns_on_path_attr_with_reason() {
    let content = "#[path = \"generated.rs\"] // reason: generated facade shim\nmod generated;";
    let results = check_source("src/lib.rs", content, false);

    assert_findings(
        &results,
        &[RuleFinding {
            severity: guardrail3_domain_report::Severity::Warn,
            title: "#[path] usage",
            message: "#[path = \"generated.rs\"] reason: generated facade shim",
            file: Some("src/lib.rs"),
            line: Some(1),
            inventory: false,
        }],
    );
}

#[test]
fn errors_on_parent_escaping_path_attr() {
    let results = check_source(
        "src/lib.rs",
        "#[path = \"../generated.rs\"]\nmod generated;",
        false,
    );

    assert_findings(
        &results,
        &[RuleFinding {
            severity: guardrail3_domain_report::Severity::Error,
            title: "#[path] escapes parent directory",
            message: "`#[path = \"../generated.rs\"]` escapes the standard module boundary.",
            file: Some("src/lib.rs"),
            line: Some(1),
            inventory: false,
        }],
    );
}

#[test]
fn skips_canonical_test_sidecar_path_wiring() {
    let content = "#[cfg(test)]\n#[path = \"rs_code_24_path_attr_tests/mod.rs\"]\nmod rs_code_24_path_attr_tests;";
    let results = check_source("src/rs_code_24_path_attr.rs", content, false);

    assert_no_hits(&results);
}

#[test]
fn skips_documented_repo_standard_test_sidecar_path_wiring() {
    let content = "#[cfg(test)]\n#[path = \"rs_code_24_path_attr_tests/mod.rs\"]\nmod tests;";
    let results = check_source("src/rs_code_24_path_attr.rs", content, false);

    assert_no_hits(&results);
}

#[test]
fn errors_on_near_miss_sidecar_path_wiring_without_cfg_test() {
    let content =
        "#[path = \"rs_code_24_path_attr_tests/mod.rs\"]\nmod rs_code_24_path_attr_tests;";
    let results = check_source("src/lib.rs", content, false);

    assert_findings(
        &results,
        &[RuleFinding {
            severity: guardrail3_domain_report::Severity::Error,
            title: "#[path] without reason",
            message: "`#[path = \"rs_code_24_path_attr_tests/mod.rs\"]` changes module resolution and requires `// reason:` on the same line.",
            file: Some("src/lib.rs"),
            line: Some(1),
            inventory: false,
        }],
    );
}

#[test]
fn errors_on_cfg_test_sidecar_path_for_another_rule_name() {
    let content = "#[cfg(test)]\n#[path = \"rs_code_99_other_rule_tests/mod.rs\"]\nmod rs_code_99_other_rule_tests;";
    let results = check_source("src/lib.rs", content, false);

    assert_findings(
        &results,
        &[RuleFinding {
            severity: guardrail3_domain_report::Severity::Error,
            title: "#[path] without reason",
            message: "`#[path = \"rs_code_99_other_rule_tests/mod.rs\"]` changes module resolution and requires `// reason:` on the same line.",
            file: Some("src/lib.rs"),
            line: Some(2),
            inventory: false,
        }],
    );
}

#[test]
fn errors_on_cfg_attr_parent_escaping_path_attr() {
    let content = "#[cfg_attr(unix, path = \"../generated.rs\")]\nmod generated;";
    let results = check_source("src/lib.rs", content, false);

    assert_findings(
        &results,
        &[RuleFinding {
            severity: guardrail3_domain_report::Severity::Error,
            title: "#[path] escapes parent directory",
            message: "`#[path = \"../generated.rs\"]` escapes the standard module boundary.",
            file: Some("src/lib.rs"),
            line: Some(1),
            inventory: false,
        }],
    );
}

#[test]
fn errors_on_cfg_attr_canonical_sidecar_path_wiring_without_reason() {
    let content =
        "#[cfg(test)]\n#[cfg_attr(unix, path = \"rs_code_24_path_attr_tests/mod.rs\")]\nmod tests;";
    let results = check_source("src/rs_code_24_path_attr.rs", content, false);

    assert_findings(
        &results,
        &[RuleFinding {
            severity: guardrail3_domain_report::Severity::Error,
            title: "#[path] without reason",
            message: "`#[path = \"rs_code_24_path_attr_tests/mod.rs\"]` changes module resolution and requires `// reason:` on the same line.",
            file: Some("src/rs_code_24_path_attr.rs"),
            line: Some(2),
            inventory: false,
        }],
    );
}
