use super::{collected_facts, dir_entry, project_tree};
use guardrail3_app_rs_family_deps_assertions::rs_deps_08_library_allowlist_present as assertions;
use guardrail3_domain_report::Severity;

#[test]
fn warns_only_for_library_crates_without_allowlists() {
    let tree = project_tree(
        vec![
            ("", dir_entry(&["apps", "packages"], &["guardrail3.toml"])),
            ("apps", dir_entry(&["api"], &[])),
            ("apps/api", dir_entry(&[], &["Cargo.toml"])),
            ("packages", dir_entry(&["core"], &[])),
            ("packages/core", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![
            (
                "guardrail3.toml",
                r#"
                    [rust.apps.api]
                    profile = "service"

                    [rust.packages]
                    profile = "library"
                "#,
            ),
            (
                "apps/api/Cargo.toml",
                r#"
                    [package]
                    name = "api"
                "#,
            ),
            (
                "packages/core/Cargo.toml",
                r#"
                    [package]
                    name = "core"
                "#,
            ),
        ],
    );
    let facts = collected_facts(&tree, &[]);
    let results = super::run_with_facts(&facts);
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            file: Some("packages/core/Cargo.toml"),
            severity: Some(Severity::Warn),
            message: Some("Library crate `core` has no `allowed_deps` policy."),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}
