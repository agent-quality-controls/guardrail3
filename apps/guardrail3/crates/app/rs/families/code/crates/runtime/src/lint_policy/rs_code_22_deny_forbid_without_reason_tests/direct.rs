use super::super::check_source;
use guardrail3_app_rs_family_code_assertions::lint_policy::rs_code_22_deny_forbid_without_reason::{
    RuleFinding, assert_findings,
};

#[test]
fn errors_on_undocumented_deny_attr() {
    let results = check_source("src/lib.rs", "#[deny(clippy::panic)]\nfn foo() {}", false);

    assert_findings(
        &results,
        &[RuleFinding::new(
            guardrail3_domain_report::Severity::Error,
            "#[deny]/#[forbid] without reason",
            "`#[deny(clippy::panic)]` changes local lint policy without documenting why. Add `// reason:` on the same line.",
            Some("src/lib.rs"),
            Some(1),
            false,
        )],
    );
}

#[test]
fn inventories_crate_level_forbid_unsafe_code() {
    let results = check_source("src/lib.rs", "#![forbid(unsafe_code)]\nfn foo() {}", false);

    assert_findings(
        &results,
        &[RuleFinding::new(
            guardrail3_domain_report::Severity::Info,
            "forbid(unsafe_code)",
            "`forbid(unsafe_code)` strengthens the local safety boundary.",
            Some("src/lib.rs"),
            Some(1),
            true,
        )],
    );
}

#[test]
fn errors_on_crate_level_deny_warnings() {
    let results = check_source("src/lib.rs", "#![deny(warnings)]\nfn foo() {}", false);

    assert_findings(
        &results,
        &[RuleFinding::new(
            guardrail3_domain_report::Severity::Error,
            "#[deny]/#[forbid] without reason",
            "`#[deny(warnings)]` changes local lint policy without documenting why. Add `// reason:` on the same line.",
            Some("src/lib.rs"),
            Some(1),
            false,
        )],
    );
}

#[test]
fn errors_on_non_inner_forbid_unsafe_code() {
    let results = check_source("src/lib.rs", "#[forbid(unsafe_code)]\nfn foo() {}", false);

    assert_findings(
        &results,
        &[RuleFinding::new(
            guardrail3_domain_report::Severity::Error,
            "#[deny]/#[forbid] without reason",
            "`#[forbid(unsafe_code)]` changes local lint policy without documenting why. Add `// reason:` on the same line.",
            Some("src/lib.rs"),
            Some(1),
            false,
        )],
    );
}

#[test]
fn errors_on_grouped_deny_lints_without_reason() {
    let results = check_source(
        "src/lib.rs",
        "#[deny(clippy::panic, clippy::expect_used)]\nfn foo() {}",
        false,
    );

    assert_findings(
        &results,
        &[
            RuleFinding::new(
                guardrail3_domain_report::Severity::Error,
                "#[deny]/#[forbid] without reason",
                "`#[deny(clippy::panic)]` changes local lint policy without documenting why. Add `// reason:` on the same line.",
                Some("src/lib.rs"),
                Some(1),
                false,
            ),
            RuleFinding::new(
                guardrail3_domain_report::Severity::Error,
                "#[deny]/#[forbid] without reason",
                "`#[deny(clippy::expect_used)]` changes local lint policy without documenting why. Add `// reason:` on the same line.",
                Some("src/lib.rs"),
                Some(1),
                false,
            ),
        ],
    );
}

#[test]
fn errors_on_trait_item_deny_attr() {
    let content = "trait Api {\n    #[deny(clippy::panic)]\n    fn run();\n}";
    let results = check_source("src/lib.rs", content, false);

    assert_findings(
        &results,
        &[RuleFinding::new(
            guardrail3_domain_report::Severity::Error,
            "#[deny]/#[forbid] without reason",
            "`#[deny(clippy::panic)]` changes local lint policy without documenting why. Add `// reason:` on the same line.",
            Some("src/lib.rs"),
            Some(2),
            false,
        )],
    );
}

#[test]
fn errors_on_cfg_attr_deny_without_reason() {
    let results = check_source(
        "src/lib.rs",
        "#[cfg_attr(unix, deny(clippy::panic))]\nfn foo() {}",
        false,
    );

    assert_findings(
        &results,
        &[RuleFinding::new(
            guardrail3_domain_report::Severity::Error,
            "#[deny]/#[forbid] without reason",
            "`#[deny(clippy::panic)]` changes local lint policy without documenting why. Add `// reason:` on the same line.",
            Some("src/lib.rs"),
            Some(1),
            false,
        )],
    );
}
