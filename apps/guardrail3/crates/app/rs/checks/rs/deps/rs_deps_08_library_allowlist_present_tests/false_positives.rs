use crate::app::rs::checks::rs::deps::run_with_facts;
use crate::app::rs::checks::rs::deps::test_support::{collected_facts, dir_entry, project_tree};
use crate::domain::report::Severity;

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
    let results = run_with_facts(&facts);
    let summary = results
        .iter()
        .filter(|result| result.id == "RS-DEPS-08")
        .map(|result| {
            (
                result.file.as_deref(),
                result.severity,
                result.message.as_str(),
                result.inventory,
            )
        })
        .collect::<Vec<_>>();

    assert_eq!(
        summary,
        vec![(
            Some("packages/core/Cargo.toml"),
            Severity::Warn,
            "Library crate `core` has no `allowed_deps` policy.",
            false,
        )]
    );
}
