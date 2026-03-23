use super::super::dependency_facts::EdgeKind;
use super::super::inputs::DependencyEdgeHexarchInput;
use super::super::test_support::{dependency_facts, dir_entry, find_edge, project_tree};
use super::check;

#[test]
fn renamed_path_dependency_violation_errors() {
    let tree = project_tree(
        vec![
            ("", dir_entry(&["apps"], &[])),
            ("apps", dir_entry(&["api"], &[])),
            ("apps/api", dir_entry(&["crates"], &["Cargo.toml"])),
            ("apps/api/crates", dir_entry(&["domain", "adapters"], &[])),
            ("apps/api/crates/domain", dir_entry(&[], &["Cargo.toml"])),
            ("apps/api/crates/adapters", dir_entry(&["http"], &[])),
            ("apps/api/crates/adapters/http", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![
            ("apps/api/Cargo.toml", "[workspace]\nmembers = [\"crates/*\", \"crates/adapters/*\"]"),
            (
                "apps/api/crates/domain/Cargo.toml",
                "[package]\nname = \"api-domain\"\n[dependencies]\nrenamed_http = { package = \"api-adapters-http\", path = \"../adapters/http\" }\n",
            ),
            ("apps/api/crates/adapters/http/Cargo.toml", "[package]\nname = \"api-adapters-http\"\n"),
        ],
    );

    let facts = dependency_facts(&tree);
    let edge = find_edge(&facts, "apps/api/crates/domain", "renamed_http", EdgeKind::Dependency);
    let mut results = Vec::new();
    check(&DependencyEdgeHexarchInput::new(edge), &mut results);

    assert_eq!(results.len(), 1, "expected one renamed dependency violation: {results:#?}");
}
