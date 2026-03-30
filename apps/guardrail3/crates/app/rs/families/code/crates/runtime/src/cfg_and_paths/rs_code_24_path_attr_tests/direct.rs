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
        &[RuleFinding::new(
            guardrail3_domain_report::Severity::Error,
            "#[path] without reason",
            "`#[path = \"generated.rs\"]` changes module resolution and requires `// reason:` on the same line.",
            Some("src/lib.rs"),
            Some(1),
            false,
        )],
    );
}

#[test]
fn warns_on_path_attr_with_reason() {
    let content = "#[path = \"generated.rs\"] // reason: generated facade shim\nmod generated;";
    let results = check_source("src/lib.rs", content, false);

    assert_findings(
        &results,
        &[RuleFinding::new(
            guardrail3_domain_report::Severity::Warn,
            "#[path] usage",
            "#[path = \"generated.rs\"] reason: generated facade shim",
            Some("src/lib.rs"),
            Some(1),
            false,
        )],
    );
}

#[test]
fn errors_on_path_attr_with_weak_reason() {
    let content = "#[path = \"generated.rs\"] // reason: temp\nmod generated;";
    let results = check_source("src/lib.rs", content, false);

    assert_findings(
        &results,
        &[RuleFinding::new(
            guardrail3_domain_report::Severity::Error,
            "#[path] reason too weak",
            "`#[path = \"generated.rs\"]` reason must be specific and at least two words. Weak reason `temp` found.",
            Some("src/lib.rs"),
            Some(1),
            false,
        )],
    );
}

#[test]
fn errors_on_noncanonical_reason_spellings() {
    let content = "#[path = \"generated.rs\"] // REASON: generated facade shim\nmod generated;\n#[path = \"generated_inline.rs\"] //reason: generated request DTO shim\nmod generated_inline;";
    let results = check_source("src/lib.rs", content, false);

    assert_findings(
        &results,
        &[
            RuleFinding::new(
                guardrail3_domain_report::Severity::Error,
                "#[path] without reason",
                "`#[path = \"generated.rs\"]` changes module resolution and requires `// reason:` on the same line.",
                Some("src/lib.rs"),
                Some(1),
                false,
            ),
            RuleFinding::new(
                guardrail3_domain_report::Severity::Error,
                "#[path] without reason",
                "`#[path = \"generated_inline.rs\"]` changes module resolution and requires `// reason:` on the same line.",
                Some("src/lib.rs"),
                Some(3),
                false,
            ),
        ],
    );
}

#[test]
fn warns_on_path_attr_with_double_slash_inside_string_literal() {
    let content =
        "#[path = \"generated//inline.rs\"] // reason: generated facade shim\nmod generated;";
    let results = check_source("src/lib.rs", content, false);

    assert_findings(
        &results,
        &[RuleFinding::new(
            guardrail3_domain_report::Severity::Warn,
            "#[path] usage",
            "#[path = \"generated//inline.rs\"] reason: generated facade shim",
            Some("src/lib.rs"),
            Some(1),
            false,
        )],
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
        &[RuleFinding::new(
            guardrail3_domain_report::Severity::Error,
            "#[path] escapes parent directory",
            "`#[path = \"../generated.rs\"]` escapes the standard module boundary.",
            Some("src/lib.rs"),
            Some(1),
            false,
        )],
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
        &[RuleFinding::new(
            guardrail3_domain_report::Severity::Error,
            "#[path] without reason",
            "`#[path = \"rs_code_24_path_attr_tests/mod.rs\"]` changes module resolution and requires `// reason:` on the same line.",
            Some("src/lib.rs"),
            Some(1),
            false,
        )],
    );
}

#[test]
fn errors_on_cfg_test_sidecar_path_for_another_rule_name() {
    let content = "#[cfg(test)]\n#[path = \"rs_code_99_other_rule_tests/mod.rs\"]\nmod rs_code_99_other_rule_tests;";
    let results = check_source("src/lib.rs", content, false);

    assert_findings(
        &results,
        &[RuleFinding::new(
            guardrail3_domain_report::Severity::Error,
            "#[path] without reason",
            "`#[path = \"rs_code_99_other_rule_tests/mod.rs\"]` changes module resolution and requires `// reason:` on the same line.",
            Some("src/lib.rs"),
            Some(2),
            false,
        )],
    );
}

#[test]
fn errors_on_cfg_attr_parent_escaping_path_attr() {
    let content = "#[cfg_attr(unix, path = \"../generated.rs\")]\nmod generated;";
    let results = check_source("src/lib.rs", content, false);

    assert_findings(
        &results,
        &[RuleFinding::new(
            guardrail3_domain_report::Severity::Error,
            "#[path] escapes parent directory",
            "`#[path = \"../generated.rs\"]` escapes the standard module boundary.",
            Some("src/lib.rs"),
            Some(1),
            false,
        )],
    );
}

#[test]
fn errors_on_cfg_attr_canonical_sidecar_path_wiring_without_reason() {
    let content =
        "#[cfg(test)]\n#[cfg_attr(unix, path = \"rs_code_24_path_attr_tests/mod.rs\")]\nmod tests;";
    let results = check_source("src/rs_code_24_path_attr.rs", content, false);

    assert_findings(
        &results,
        &[RuleFinding::new(
            guardrail3_domain_report::Severity::Error,
            "#[path] without reason",
            "`#[path = \"rs_code_24_path_attr_tests/mod.rs\"]` changes module resolution and requires `// reason:` on the same line.",
            Some("src/rs_code_24_path_attr.rs"),
            Some(2),
            false,
        )],
    );
}
