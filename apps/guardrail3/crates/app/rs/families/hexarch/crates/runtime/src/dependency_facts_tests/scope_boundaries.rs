use super::super::collect_for_test_tree as dependency_facts;
use guardrail3_app_rs_family_hexarch_assertions::dependency_facts as assertions;
use super::{dir_entry, project_tree};

#[test]
fn fixture_workspace_outside_app_crates_tree_is_ignored() {
    let tree = project_tree(
        vec![
            ("", dir_entry(&["apps"], &[])),
            ("apps", dir_entry(&["api"], &[])),
            (
                "apps/api",
                dir_entry(&["crates", "tests"], &["Cargo.toml"]),
            ),
            ("apps/api/crates", dir_entry(&["domain"], &[])),
            ("apps/api/crates/domain", dir_entry(&["core"], &[])),
            (
                "apps/api/crates/domain/core",
                dir_entry(&[], &["Cargo.toml"]),
            ),
            ("apps/api/tests", dir_entry(&["fixtures"], &[])),
            ("apps/api/tests/fixtures", dir_entry(&["shadow"], &[])),
            (
                "apps/api/tests/fixtures/shadow",
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
                "[package]\nname = \"api-domain-core\"\n",
            ),
            (
                "apps/api/tests/fixtures/shadow/Cargo.toml",
                "[workspace]\nmembers = []\n[workspace.dependencies]\nshadow = { path = \"../../../../packages/shadow\" }\n",
            ),
        ],
    );

    let facts = dependency_facts(&tree);
    let workspace_roots = facts
        .workspaces
        .iter()
        .map(|workspace| workspace.root_rel_dir.as_str())
        .collect::<Vec<_>>();
    assert_eq!(workspace_roots, vec!["apps/api"]);
    assertions::assert_member_count(&facts.members, 1);
}
