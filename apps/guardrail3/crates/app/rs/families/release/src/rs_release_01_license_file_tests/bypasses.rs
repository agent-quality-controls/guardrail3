use guardrail3_domain_report::Severity;

use super::super::super::check as run_family;
use super::super::super::test_support::{StubToolChecker, dir_entry, project_tree, temp_root};
use super::super::super::test_support::{repo_facts, repo_input};
use super::super::check;

#[test]
fn errors_when_no_license_material_exists() {
    let facts = repo_facts();
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-RELEASE-01");
    assert_eq!(results[0].severity, Severity::Error);
    assert!(!results[0].inventory);
    assert_eq!(results[0].file.as_deref(), Some("Cargo.toml"));
}

#[test]
fn errors_when_license_path_is_nested_or_not_whitelisted() {
    for rel_path in ["docs/LICENSE", "LICENSE.txt"] {
        let mut facts = repo_facts();
        facts.license_rel_path = Some(rel_path.to_owned());
        let input = repo_input(&facts);
        let mut results = Vec::new();

        check(&input, &mut results);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "RS-RELEASE-01");
        assert_eq!(results[0].severity, Severity::Error);
        assert!(!results[0].inventory);
        assert_eq!(results[0].file.as_deref(), Some("Cargo.toml"));
    }
}

#[test]
fn does_not_error_when_allowed_root_license_exists_beside_distracting_near_misses() {
    let root = temp_root("release-license-root-plus-near-miss");
    let tree = project_tree(
        vec![(
            "",
            dir_entry(&[], &["Cargo.toml", "LICENSE", "LICENSE.txt", "README.md"]),
        )],
        vec![
            (
                "Cargo.toml",
                r#"
[package]
name = "example"
version = "0.1.0"
description = "example"
license = "MIT"
repository = "https://example.com/repo"
"#,
            ),
            ("LICENSE", "canonical license text\n"),
            ("LICENSE.txt", "distracting near miss\n"),
            ("README.md", "# Example\n\nREADME\n"),
        ],
        root,
    );
    let results = run_family(&tree, &StubToolChecker::new(true), false);

    assert!(results.iter().any(|result| {
        result.id == "RS-RELEASE-01"
            && result.severity == Severity::Info
            && result.inventory
            && result.file.as_deref() == Some("LICENSE")
    }));
}
