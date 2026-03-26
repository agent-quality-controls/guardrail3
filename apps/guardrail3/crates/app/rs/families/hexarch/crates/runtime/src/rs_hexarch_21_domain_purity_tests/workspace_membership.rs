use std::collections::BTreeSet;

use super::super::super::collect_dependency_facts_from_tree_for_tests as dependency_facts;
use super::super::super::dependency_facts::EdgeKind;
use super::super::super::inputs::MemberDependencyHexarchInput;
use super::super::check;
use test_support::{dir_entry, project_tree};

#[test]
fn omitted_same_app_pure_layer_targets_do_not_count_as_allowed_internal_deps() {
    let tree = project_tree(
        vec![
            ("", dir_entry(&["apps"], &[])),
            ("apps", dir_entry(&["api"], &[])),
            ("apps/api", dir_entry(&["crates"], &["Cargo.toml"])),
            ("apps/api/crates", dir_entry(&["domain", "ports"], &[])),
            (
                "apps/api/crates/domain",
                dir_entry(&["core", "shared"], &[]),
            ),
            (
                "apps/api/crates/domain/core",
                dir_entry(&[], &["Cargo.toml"]),
            ),
            (
                "apps/api/crates/domain/shared",
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
                "[workspace]\nmembers = [\"crates/domain/core\"]\n",
            ),
            (
                "apps/api/crates/domain/core/Cargo.toml",
                "[package]\nname = \"api-domain-core\"\n[dependencies]\napi-domain-shared = { path = \"../shared\" }\napi-ports-repo = { path = \"../../ports/repo\" }\n",
            ),
            (
                "apps/api/crates/domain/shared/Cargo.toml",
                "[package]\nname = \"api-domain-shared\"\n",
            ),
            (
                "apps/api/crates/ports/repo/Cargo.toml",
                "[package]\nname = \"api-ports-repo\"\n",
            ),
        ],
    );

    let facts = dependency_facts(&tree);
    let member = facts
        .members
        .iter()
        .find(|member| member.rel_dir == "apps/api/crates/domain/core")
        .expect("domain member");
    let edges = facts
        .edges
        .iter()
        .filter(|edge| edge.source_rel_dir == member.rel_dir && edge.kind == EdgeKind::Dependency)
        .collect();
    let mut results = Vec::new();
    check(
        &MemberDependencyHexarchInput::new(member, edges),
        &mut results,
    );

    let actual_titles = results
        .iter()
        .map(|result| result.title.clone())
        .collect::<BTreeSet<_>>();
    let expected_titles =
        ["domain crate `api-domain-core` depends on non-workspace pure-layer crate".to_owned()]
            .into_iter()
            .collect::<BTreeSet<_>>();
    assert_eq!(
        actual_titles, expected_titles,
        "omitted same-app pure-layer targets must not fail open as allowed internal deps: {results:#?}"
    );
    assert_eq!(
        results.len(),
        2,
        "both omitted pure-layer edges should be reported: {results:#?}"
    );
}
