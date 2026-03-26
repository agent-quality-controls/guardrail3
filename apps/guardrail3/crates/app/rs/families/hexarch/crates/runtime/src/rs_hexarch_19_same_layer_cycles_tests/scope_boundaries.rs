use super::super::super::collect_dependency_facts_from_tree_for_tests as dependency_facts;
use test_support::{dir_entry, project_tree};

#[test]
fn target_specific_same_layer_cycle_is_filtered_out() {
    let tree = project_tree(
        vec![
            ("", dir_entry(&["apps"], &[])),
            ("apps", dir_entry(&["api"], &[])),
            ("apps/api", dir_entry(&["crates"], &["Cargo.toml"])),
            ("apps/api/crates", dir_entry(&["domain"], &[])),
            ("apps/api/crates/domain", dir_entry(&["a", "b"], &[])),
            ("apps/api/crates/domain/a", dir_entry(&[], &["Cargo.toml"])),
            ("apps/api/crates/domain/b", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![
            (
                "apps/api/Cargo.toml",
                "[workspace]\nmembers = [\"crates/domain/a\", \"crates/domain/b\"]\n",
            ),
            (
                "apps/api/crates/domain/a/Cargo.toml",
                "[package]\nname = \"api-domain-a\"\n[target.'cfg(unix)'.dependencies]\napi-domain-b = { path = \"../b\" }\n",
            ),
            (
                "apps/api/crates/domain/b/Cargo.toml",
                "[package]\nname = \"api-domain-b\"\n[target.'cfg(windows)'.dependencies]\napi-domain-a = { path = \"../a\" }\n",
            ),
        ],
    );

    let facts = dependency_facts(&tree);
    assert!(
        facts.cycles.is_empty(),
        "target-specific edges should not fabricate same-layer cycles: {facts:#?}"
    );
}

#[test]
fn omitted_same_layer_member_stays_out_of_cycle_detection() {
    let tree = project_tree(
        vec![
            ("", dir_entry(&["apps"], &[])),
            ("apps", dir_entry(&["api"], &[])),
            ("apps/api", dir_entry(&["crates"], &["Cargo.toml"])),
            ("apps/api/crates", dir_entry(&["domain"], &[])),
            ("apps/api/crates/domain", dir_entry(&["a", "b"], &[])),
            ("apps/api/crates/domain/a", dir_entry(&[], &["Cargo.toml"])),
            ("apps/api/crates/domain/b", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![
            (
                "apps/api/Cargo.toml",
                "[workspace]\nmembers = [\"crates/domain/a\"]\n",
            ),
            (
                "apps/api/crates/domain/a/Cargo.toml",
                "[package]\nname = \"api-domain-a\"\n[dependencies]\napi-domain-b = { path = \"../b\" }\n",
            ),
            (
                "apps/api/crates/domain/b/Cargo.toml",
                "[package]\nname = \"api-domain-b\"\n[dependencies]\napi-domain-a = { path = \"../a\" }\n",
            ),
        ],
    );

    let facts = dependency_facts(&tree);
    assert!(
        facts.cycles.is_empty(),
        "omitted same-layer members should stay out of cycle detection: {facts:#?}"
    );
}
