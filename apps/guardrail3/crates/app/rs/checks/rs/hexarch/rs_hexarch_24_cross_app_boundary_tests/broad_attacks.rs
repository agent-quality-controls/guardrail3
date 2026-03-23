use std::collections::BTreeSet;

use super::super::super::test_support::{dependency_facts, dir_entry, project_tree};
use super::super::check;

#[test]
fn cross_app_edges_error_and_same_app_edges_do_not() {
    let tree = project_tree(
        vec![
            ("", dir_entry(&["apps"], &[])),
            ("apps", dir_entry(&["api", "worker"], &[])),
            ("apps/api", dir_entry(&["crates"], &["Cargo.toml"])),
            ("apps/api/crates", dir_entry(&["app", "domain"], &[])),
            ("apps/api/crates/app", dir_entry(&["core"], &[])),
            ("apps/api/crates/app/core", dir_entry(&[], &["Cargo.toml"])),
            ("apps/api/crates/domain", dir_entry(&["types"], &[])),
            (
                "apps/api/crates/domain/types",
                dir_entry(&[], &["Cargo.toml"]),
            ),
            ("apps/worker", dir_entry(&["crates"], &["Cargo.toml"])),
            ("apps/worker/crates", dir_entry(&["domain"], &[])),
            ("apps/worker/crates/domain", dir_entry(&["jobs"], &[])),
            (
                "apps/worker/crates/domain/jobs",
                dir_entry(&[], &["Cargo.toml"]),
            ),
            ("apps/worker/crates/app", dir_entry(&["processor"], &[])),
            (
                "apps/worker/crates/app/processor",
                dir_entry(&[], &["Cargo.toml"]),
            ),
        ],
        vec![
            (
                "apps/api/Cargo.toml",
                "[workspace]\nmembers = [\"crates/*\", \"crates/*/*\"]\n",
            ),
            (
                "apps/worker/Cargo.toml",
                "[workspace]\nmembers = [\"crates/*\", \"crates/*/*\"]\n",
            ),
            (
                "apps/api/crates/app/core/Cargo.toml",
                "[package]\nname = \"api-app-core\"\n\n[dependencies]\napi-domain-types = { path = \"../../domain/types\" }\nworker-domain-jobs = { path = \"../../../../worker/crates/domain/jobs\" }\n\n[dev-dependencies]\nworker-app-processor = { path = \"../../../../worker/crates/app/processor\" }\n\n[build-dependencies]\nworker-ports-outbound-queue = { path = \"../../../../worker/crates/ports/outbound/queue\" }\n\n[target.'cfg(unix)'.dependencies]\nworker-domain-jobs-target = { path = \"../../../../worker/crates/domain/jobs\" }\n",
            ),
            (
                "apps/api/crates/domain/types/Cargo.toml",
                "[package]\nname = \"api-domain-types\"\n",
            ),
            (
                "apps/worker/crates/domain/jobs/Cargo.toml",
                "[package]\nname = \"worker-domain-jobs\"\n",
            ),
            (
                "apps/worker/crates/app/processor/Cargo.toml",
                "[package]\nname = \"worker-app-processor\"\n",
            ),
            (
                "apps/worker/crates/ports/outbound/queue/Cargo.toml",
                "[package]\nname = \"worker-ports-outbound-queue\"\n",
            ),
        ],
    );

    let facts = dependency_facts(&tree);
    let mut results = Vec::new();
    for edge in &facts.edges {
        check(
            &super::super::super::inputs::DependencyEdgeHexarchInput::new(edge),
            &mut results,
        );
    }

    assert_eq!(
        results.len(),
        4,
        "expected one hit per cross-app dependency section: {results:#?}"
    );
    let actual_files = results
        .iter()
        .filter_map(|result| result.file.clone())
        .collect::<BTreeSet<_>>();
    let expected_files = ["apps/api/crates/app/core/Cargo.toml".to_owned()]
        .into_iter()
        .collect::<BTreeSet<_>>();
    assert_eq!(
        actual_files, expected_files,
        "unexpected cross-app hit set: {results:#?}"
    );
}
