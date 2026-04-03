use super::helpers::results_for_cycles_for_test as results_for_cycles;
use super::{dir_entry, project_tree};
use guardrail3_app_rs_family_hexarch_assertions::dependency_integrity::rs_hexarch_19_same_layer_cycles as assertions;

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

    let (cycle_layers, _results) = results_for_cycles(&tree);
    assertions::assert_cycle_layers(&cycle_layers, 0, &[]);
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

    let (cycle_layers, _results) = results_for_cycles(&tree);
    assertions::assert_cycle_layers(&cycle_layers, 0, &[]);
}
