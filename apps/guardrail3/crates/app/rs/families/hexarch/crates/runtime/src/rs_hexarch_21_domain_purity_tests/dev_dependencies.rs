use super::super::super::collect_dependency_facts_from_tree_for_tests as dependency_facts;
use super::super::super::dependency_facts::EdgeKind;
use super::super::super::inputs::MemberDependencyHexarchInput;
use super::super::check;
use test_support::{dir_entry, project_tree};

#[test]
fn dev_dependencies_do_not_trigger_domain_purity() {
    let tree = project_tree(
        vec![
            ("", dir_entry(&["apps"], &[])),
            ("apps", dir_entry(&["api"], &[])),
            ("apps/api", dir_entry(&["crates"], &["Cargo.toml"])),
            (
                "apps/api/crates",
                dir_entry(&["domain", "ports", "adapters"], &[]),
            ),
            ("apps/api/crates/domain", dir_entry(&["core"], &[])),
            (
                "apps/api/crates/domain/core",
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
                "apps/api/crates/domain/core/Cargo.toml",
                "[package]\nname = \"api-domain-core\"\n[dev-dependencies]\nserde = \"1\"\napi-ports-repo = { path = \"../../ports/repo\" }\napi-adapters-http = { path = \"../../adapters/http\" }\n",
            ),
            (
                "apps/api/crates/ports/repo/Cargo.toml",
                "[package]\nname = \"api-ports-repo\"\n",
            ),
            (
                "apps/api/crates/adapters/http/Cargo.toml",
                "[package]\nname = \"api-adapters-http\"\n",
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
        .filter(|edge| {
            edge.source_rel_dir == member.rel_dir && edge.kind == EdgeKind::DevDependency
        })
        .collect();
    let mut results = Vec::new();
    check(
        &MemberDependencyHexarchInput::new(member, edges),
        &mut results,
    );

    assert!(
        results.is_empty(),
        "dev dependencies should remain out of scope for domain purity: {results:#?}"
    );
}
