use super::super::dependency_facts::EdgeKind;
use super::super::inputs::MemberDependencyHexarchInput;
use super::super::test_support::{dependency_facts, dir_entry, project_tree};
use super::check;

#[test]
fn domain_external_tokio_is_error() {
    let tree = project_tree(
        vec![
            ("", dir_entry(&["apps"], &["guardrail3.toml"])),
            ("apps", dir_entry(&["api"], &[])),
            ("apps/api", dir_entry(&["crates"], &["Cargo.toml"])),
            ("apps/api/crates", dir_entry(&["domain"], &[])),
            ("apps/api/crates/domain", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![
            ("guardrail3.toml", "[rust.apps.api]\nprofile = \"service\"\n"),
            ("apps/api/Cargo.toml", "[workspace]\nmembers = [\"crates/*\"]"),
            (
                "apps/api/crates/domain/Cargo.toml",
                "[package]\nname = \"api-domain\"\n[dependencies]\ntokio = \"1\"\n",
            ),
        ],
    );

    let facts = dependency_facts(&tree);
    let member = facts
        .members
        .iter()
        .find(|member| member.rel_dir == "apps/api/crates/domain")
        .expect("domain member");
    let edges = facts
        .edges
        .iter()
        .filter(|edge| edge.source_rel_dir == member.rel_dir && edge.kind == EdgeKind::Dependency)
        .collect();
    let mut results = Vec::new();
    check(&MemberDependencyHexarchInput::new(member, edges), &mut results);

    assert_eq!(results.len(), 1, "expected one domain-purity error: {results:#?}");
    assert!(results[0].message.contains("tokio"));
}
