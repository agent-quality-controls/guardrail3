use std::collections::BTreeSet;

use super::super::super::dependency_facts::EdgeKind;
use super::super::super::inputs::DependencyEdgeHexarchInput;
use super::super::super::test_support::{dependency_facts, dir_entry, project_tree};
use super::super::check;

#[test]
fn forbidden_renamed_edges_error_and_unrenamed_edges_do_not() {
    let tree = project_tree(
        vec![
            ("", dir_entry(&["apps"], &[])),
            ("apps", dir_entry(&["api"], &[])),
            ("apps/api", dir_entry(&["crates"], &["Cargo.toml"])),
            (
                "apps/api/crates",
                dir_entry(&["domain", "ports", "adapters"], &[]),
            ),
            ("apps/api/crates/domain", dir_entry(&["types"], &[])),
            (
                "apps/api/crates/domain/types",
                dir_entry(&[], &["Cargo.toml"]),
            ),
            ("apps/api/crates/ports", dir_entry(&["repo"], &[])),
            (
                "apps/api/crates/ports/repo",
                dir_entry(&[], &["Cargo.toml"]),
            ),
            ("apps/api/crates/adapters", dir_entry(&["http"], &[])),
            (
                "apps/api/crates/adapters/http",
                dir_entry(&[], &["Cargo.toml"]),
            ),
        ],
        vec![
            (
                "apps/api/Cargo.toml",
                "[workspace]\nmembers = [\"crates/domain/*\", \"crates/ports/*\", \"crates/adapters/*\"]\n",
            ),
            (
                "apps/api/crates/domain/types/Cargo.toml",
                "[package]\nname = \"api-domain-types\"\n[dependencies]\nrenamed_http = { package = \"api-adapters-http\", path = \"../../adapters/http\" }\n",
            ),
            (
                "apps/api/crates/ports/repo/Cargo.toml",
                "[package]\nname = \"api-ports-repo\"\n[dependencies]\nrenamed_http = { package = \"api-adapters-http\", path = \"../../adapters/http\" }\n",
            ),
            (
                "apps/api/crates/adapters/http/Cargo.toml",
                "[package]\nname = \"api-adapters-http\"\n",
            ),
        ],
    );

    let facts = dependency_facts(&tree);
    let mut results = Vec::new();
    for edge in facts
        .edges
        .iter()
        .filter(|edge| edge.kind == EdgeKind::Dependency)
    {
        check(&DependencyEdgeHexarchInput::new(edge), &mut results);
    }

    let actual_files = results
        .iter()
        .filter_map(|result| result.file.clone())
        .collect::<BTreeSet<_>>();
    let expected_files = [
        "apps/api/crates/domain/types/Cargo.toml".to_owned(),
        "apps/api/crates/ports/repo/Cargo.toml".to_owned(),
    ]
    .into_iter()
    .collect::<BTreeSet<_>>();

    assert_eq!(
        actual_files, expected_files,
        "unexpected renamed-direction hit set: {results:#?}"
    );
}
