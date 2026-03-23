use std::collections::BTreeSet;

use super::super::super::dependency_facts::EdgeKind;
use super::super::super::inputs::DependencyEdgeHexarchInput;
use super::super::super::test_support::{dependency_facts, dir_entry, project_tree};
use super::super::check;

#[test]
fn path_dependencies_are_inventoried_with_exact_source_set() {
    let tree = project_tree(
        vec![
            ("", dir_entry(&["apps"], &[])),
            ("apps", dir_entry(&["api"], &[])),
            ("apps/api", dir_entry(&["crates"], &["Cargo.toml"])),
            (
                "apps/api/crates",
                dir_entry(&["app", "domain", "ports"], &[]),
            ),
            ("apps/api/crates/app", dir_entry(&["core"], &[])),
            ("apps/api/crates/app/core", dir_entry(&[], &["Cargo.toml"])),
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
        ],
        vec![
            (
                "apps/api/Cargo.toml",
                "[workspace]\nmembers = [\"crates/app/*\", \"crates/domain/*\", \"crates/ports/*\"]\n",
            ),
            (
                "apps/api/crates/app/core/Cargo.toml",
                "[package]\nname = \"api-app-core\"\n[dependencies]\napi-domain-types = { path = \"../../domain/types\" }\napi-ports-repo = { path = \"../../ports/repo\" }\n",
            ),
            (
                "apps/api/crates/domain/types/Cargo.toml",
                "[package]\nname = \"api-domain-types\"\n",
            ),
            (
                "apps/api/crates/ports/repo/Cargo.toml",
                "[package]\nname = \"api-ports-repo\"\n",
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

    let actual_messages = results
        .iter()
        .map(|result| result.message.clone())
        .collect::<BTreeSet<_>>();
    let expected_messages = [
        "`apps/api/crates/app/core` depends on `api-domain-types` via `dependencies` resolved to `apps/api/crates/domain/types`.".to_owned(),
        "`apps/api/crates/app/core` depends on `api-ports-repo` via `dependencies` resolved to `apps/api/crates/ports/repo`.".to_owned(),
    ]
    .into_iter()
    .collect::<BTreeSet<_>>();

    assert_eq!(
        actual_messages, expected_messages,
        "unexpected inventory results: {results:#?}"
    );
    assert!(results.iter().all(|result| result.inventory));
}
