use super::super::super::dependency_facts::EdgeKind;
use super::super::super::inputs::MemberDependencyHexarchInput;
use super::super::super::test_support::{dependency_facts, dir_entry, project_tree};
use super::super::check;

#[test]
fn domain_and_ports_path_deps_do_not_trigger_domain_purity() {
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
                "[workspace]\nmembers = [\"crates/domain/*\", \"crates/ports/*\"]\n",
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

    assert!(
        results.is_empty(),
        "pure domain and ports path dependencies should not trigger domain purity: {results:#?}"
    );
}
