use std::collections::BTreeSet;

use super::super::super::collect_dependency_facts_from_tree_for_tests as dependency_facts;
use super::super::super::inputs::DependencyEdgeHexarchInput;
use super::super::check;
use test_support::{dir_entry, project_tree};

#[test]
fn forbidden_target_sections_error_and_allowed_target_sections_do_not() {
    let tree = project_tree(
        vec![
            ("", dir_entry(&["apps"], &[])),
            ("apps", dir_entry(&["api"], &[])),
            ("apps/api", dir_entry(&["crates"], &["Cargo.toml"])),
            (
                "apps/api/crates",
                dir_entry(&["domain", "app", "adapters"], &[]),
            ),
            (
                "apps/api/crates/domain",
                dir_entry(&["types", "helper"], &[]),
            ),
            (
                "apps/api/crates/domain/types",
                dir_entry(&[], &["Cargo.toml"]),
            ),
            (
                "apps/api/crates/domain/helper",
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
                "[workspace]\nmembers = [\"crates/domain/*\", \"crates/app/*\", \"crates/adapters/*\"]\n",
            ),
            (
                "apps/api/crates/domain/types/Cargo.toml",
                "[package]\nname = \"api-domain-types\"\n[target.'cfg(unix)'.dependencies]\napi-domain-helper = { path = \"../helper\" }\n",
            ),
            (
                "apps/api/crates/domain/helper/Cargo.toml",
                "[package]\nname = \"api-domain-helper\"\n",
            ),
            (
                "apps/api/crates/app/core/Cargo.toml",
                "[package]\nname = \"api-app-core\"\n[target.'cfg(unix)'.dependencies]\napi-adapters-http = { path = \"../../adapters/http\" }\n[target.'cfg(windows)'.dev-dependencies]\napi-adapters-http = { path = \"../../adapters/http\" }\n[target.'cfg(target_os = \"linux\")'.build-dependencies]\napi-adapters-http = { path = \"../../adapters/http\" }\n",
            ),
            (
                "apps/api/crates/adapters/http/Cargo.toml",
                "[package]\nname = \"api-adapters-http\"\n",
            ),
        ],
    );

    let facts = dependency_facts(&tree);
    let mut results = Vec::new();
    for edge in facts.edges.iter().filter(|edge| edge.kind.is_target()) {
        check(&DependencyEdgeHexarchInput::new(edge), &mut results);
    }

    let actual_files = results
        .iter()
        .filter_map(|result| result.file.clone())
        .collect::<BTreeSet<_>>();
    let expected_files = ["apps/api/crates/app/core/Cargo.toml".to_owned()]
        .into_iter()
        .collect::<BTreeSet<_>>();

    assert_eq!(
        results.len(),
        3,
        "expected one target-direction error per forbidden target section: {results:#?}"
    );
    assert_eq!(
        actual_files, expected_files,
        "unexpected target-direction hit set: {results:#?}"
    );
}
