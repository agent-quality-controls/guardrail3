use std::collections::BTreeSet;

use super::super::{run_domain_purity_case, DomainPurityEdgeKindForTest};
use test_support::{dir_entry, project_tree};

#[test]
fn inherited_workspace_externals_still_trigger_domain_purity() {
    let tree = project_tree(
        vec![
            ("", dir_entry(&["apps"], &["guardrail3.toml"])),
            ("apps", dir_entry(&["api"], &[])),
            ("apps/api", dir_entry(&["crates"], &["Cargo.toml"])),
            ("apps/api/crates", dir_entry(&["domain"], &[])),
            ("apps/api/crates/domain", dir_entry(&["core"], &[])),
            (
                "apps/api/crates/domain/core",
                dir_entry(&[], &["Cargo.toml"]),
            ),
        ],
        vec![
            (
                "guardrail3.toml",
                "[rust.apps.api]\nprofile = \"service\"\n",
            ),
            (
                "apps/api/Cargo.toml",
                "[workspace]\nmembers = [\"crates/domain/*\"]\n[workspace.dependencies]\ntokio = \"1\"\n",
            ),
            (
                "apps/api/crates/domain/core/Cargo.toml",
                "[package]\nname = \"api-domain-core\"\n[dependencies]\ntokio = { workspace = true, optional = true }\n",
            ),
        ],
    );

    let results = run_domain_purity_case(
        &tree,
        "apps/api/crates/domain/core",
        DomainPurityEdgeKindForTest::Dependency,
    );

    let actual_titles = results
        .iter()
        .map(|result| result.title.clone())
        .collect::<BTreeSet<_>>();
    let expected_titles = [
        "domain crate `api-domain-core` depends on disallowed external crate `tokio`".to_owned(),
    ]
    .into_iter()
    .collect::<BTreeSet<_>>();
    assert_eq!(
        actual_titles, expected_titles,
        "workspace-inherited externals should still be checked by domain purity: {results:#?}"
    );
}
