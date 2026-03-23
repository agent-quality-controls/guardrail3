use super::super::dependency_facts::EdgeKind;
use super::super::inputs::DependencyEdgeHexarchInput;
use super::super::test_support::{dependency_facts, dir_entry, find_edge, project_tree};
use super::check;

#[test]
fn dev_dependency_violation_warns() {
    let tree = project_tree(
        vec![
            ("", dir_entry(&["apps"], &[])),
            ("apps", dir_entry(&["api"], &[])),
            ("apps/api", dir_entry(&["crates"], &["Cargo.toml"])),
            ("apps/api/crates", dir_entry(&["domain", "adapters"], &[])),
            ("apps/api/crates/domain", dir_entry(&[], &["Cargo.toml"])),
            ("apps/api/crates/adapters", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![
            ("apps/api/Cargo.toml", "[workspace]\nmembers = [\"crates/*\"]"),
            (
                "apps/api/crates/domain/Cargo.toml",
                "[package]\nname = \"api-domain\"\n[dev-dependencies]\napi-adapters = { path = \"../adapters\" }\n",
            ),
            ("apps/api/crates/adapters/Cargo.toml", "[package]\nname = \"api-adapters\"\n"),
        ],
    );

    let facts = dependency_facts(&tree);
    let edge = find_edge(&facts, "apps/api/crates/domain", "api-adapters", EdgeKind::DevDependency);
    let mut results = Vec::new();
    check(&DependencyEdgeHexarchInput::new(edge), &mut results);

    assert_eq!(results.len(), 1, "expected one dev-direction warning: {results:#?}");
    assert!(matches!(results[0].severity, crate::domain::report::Severity::Warn));
}
