use std::collections::BTreeSet;

use super::super::{run_domain_purity_case, DomainPurityEdgeKindForTest};
use test_support::{dir_entry, project_tree};

#[test]
fn build_dependencies_are_in_scope_for_domain_purity() {
    let tree = project_tree(
        vec![
            ("", dir_entry(&["apps"], &[])),
            ("apps", dir_entry(&["api"], &[])),
            ("apps/api", dir_entry(&["crates"], &["Cargo.toml"])),
            ("apps/api/crates", dir_entry(&["domain", "adapters"], &[])),
            ("apps/api/crates/domain", dir_entry(&["core"], &[])),
            (
                "apps/api/crates/domain/core",
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
                "[workspace]\nmembers = [\"crates/domain/*\", \"crates/adapters/*\"]\n",
            ),
            (
                "apps/api/crates/domain/core/Cargo.toml",
                "[package]\nname = \"api-domain-core\"\n[build-dependencies]\ntokio = \"1\"\napi-adapters-http = { path = \"../../adapters/http\" }\n",
            ),
            (
                "apps/api/crates/adapters/http/Cargo.toml",
                "[package]\nname = \"api-adapters-http\"\n",
            ),
        ],
    );

    let results = run_domain_purity_case(
        &tree,
        "apps/api/crates/domain/core",
        DomainPurityEdgeKindForTest::BuildDependency,
    );

    let actual_titles = results
        .iter()
        .map(|result| result.title.clone())
        .collect::<BTreeSet<_>>();
    let expected_titles = [
        "domain crate `api-domain-core` depends on disallowed external crate `tokio`".to_owned(),
        "domain crate `api-domain-core` depends on non-pure layer".to_owned(),
    ]
    .into_iter()
    .collect::<BTreeSet<_>>();
    assert_eq!(
        actual_titles, expected_titles,
        "build dependencies should be treated as real domain dependencies: {results:#?}"
    );
}
