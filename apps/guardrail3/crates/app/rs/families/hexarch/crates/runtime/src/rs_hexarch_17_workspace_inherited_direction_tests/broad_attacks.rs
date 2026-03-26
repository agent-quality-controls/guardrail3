use std::collections::BTreeSet;

use super::super::results_for_dependency_edges_for_test as results_for_dependency_edges;
use test_support::{dir_entry, project_tree};

#[test]
fn forbidden_workspace_inherited_edges_error_and_allowed_ones_do_not() {
    let tree = project_tree(
        vec![
            ("", dir_entry(&["apps"], &[])),
            ("apps", dir_entry(&["api"], &[])),
            ("apps/api", dir_entry(&["crates"], &["Cargo.toml"])),
            (
                "apps/api/crates",
                dir_entry(&["domain", "ports", "app", "adapters"], &[]),
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
            ("apps/api/crates/app", dir_entry(&["core"], &[])),
            ("apps/api/crates/app/core", dir_entry(&[], &["Cargo.toml"])),
            ("apps/api/crates/adapters", dir_entry(&["http"], &[])),
            (
                "apps/api/crates/adapters/http",
                dir_entry(&[], &["Cargo.toml"]),
            ),
        ],
        vec![
            (
                "apps/api/Cargo.toml",
                "[workspace]\nmembers = [\"crates/domain/*\", \"crates/ports/*\", \"crates/app/*\", \"crates/adapters/*\"]\n[workspace.dependencies]\napi-app-core = { path = \"crates/app/core\" }\napi-adapters-http = { path = \"crates/adapters/http\" }\napi-domain-types = { path = \"crates/domain/types\" }\n",
            ),
            (
                "apps/api/crates/domain/types/Cargo.toml",
                "[package]\nname = \"api-domain-types\"\n[dependencies]\napi-app-core = { workspace = true }\napi-adapters-http = { workspace = true }\n",
            ),
            (
                "apps/api/crates/ports/repo/Cargo.toml",
                "[package]\nname = \"api-ports-repo\"\n[dependencies]\napi-domain-types = { workspace = true }\napi-adapters-http = { workspace = true }\n",
            ),
            (
                "apps/api/crates/app/core/Cargo.toml",
                "[package]\nname = \"api-app-core\"\n",
            ),
            (
                "apps/api/crates/adapters/http/Cargo.toml",
                "[package]\nname = \"api-adapters-http\"\n",
            ),
        ],
    );

    let results = results_for_dependency_edges(&tree);

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
        "unexpected inherited-direction hit set: {results:#?}"
    );
}
