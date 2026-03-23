use super::super::dependency_facts::EdgeKind;
use super::super::inputs::DependencyEdgeHexarchInput;
use super::super::test_support::{dependency_facts, dir_entry, find_edge, project_tree};
use super::check;

#[test]
fn path_dependency_crossing_app_boundaries_errors() {
    let tree = project_tree(
        vec![
            ("", dir_entry(&["apps"], &[])),
            ("apps", dir_entry(&["api", "worker"], &[])),
            ("apps/api", dir_entry(&["crates"], &["Cargo.toml"])),
            ("apps/api/crates", dir_entry(&["app"], &[])),
            ("apps/api/crates/app", dir_entry(&[], &["Cargo.toml"])),
            ("apps/worker", dir_entry(&["crates"], &["Cargo.toml"])),
            ("apps/worker/crates", dir_entry(&["domain"], &[])),
            ("apps/worker/crates/domain", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![
            ("apps/api/Cargo.toml", "[workspace]\nmembers = [\"crates/*\"]"),
            ("apps/worker/Cargo.toml", "[workspace]\nmembers = [\"crates/*\"]"),
            (
                "apps/api/crates/app/Cargo.toml",
                "[package]\nname = \"api-app\"\n[dependencies]\nworker-domain = { path = \"../../../worker/crates/domain\" }\n",
            ),
            ("apps/worker/crates/domain/Cargo.toml", "[package]\nname = \"worker-domain\"\n"),
        ],
    );

    let facts = dependency_facts(&tree);
    let edge = find_edge(&facts, "apps/api/crates/app", "worker-domain", EdgeKind::Dependency);
    let mut results = Vec::new();
    check(&DependencyEdgeHexarchInput::new(edge), &mut results);

    assert_eq!(results.len(), 1, "expected one cross-app error: {results:#?}");
}
