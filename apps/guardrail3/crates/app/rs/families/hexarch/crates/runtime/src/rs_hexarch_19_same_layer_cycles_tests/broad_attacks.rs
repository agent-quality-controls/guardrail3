use std::collections::BTreeSet;

use super::super::results_for_cycles_for_test as results_for_cycles;
use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_19_same_layer_cycles as assertions;
use super::{dir_entry, project_tree};

#[test]
fn same_layer_cycle_is_reported_once_even_with_mixed_layer_cycle_present() {
    let tree = project_tree(
        vec![
            ("", dir_entry(&["apps"], &[])),
            ("apps", dir_entry(&["api"], &[])),
            ("apps/api", dir_entry(&["crates"], &["Cargo.toml"])),
            (
                "apps/api/crates",
                dir_entry(&["domain", "app", "ports"], &[]),
            ),
            ("apps/api/crates/domain", dir_entry(&["a", "b", "c"], &[])),
            ("apps/api/crates/domain/a", dir_entry(&[], &["Cargo.toml"])),
            ("apps/api/crates/domain/b", dir_entry(&[], &["Cargo.toml"])),
            ("apps/api/crates/domain/c", dir_entry(&[], &["Cargo.toml"])),
            ("apps/api/crates/app", dir_entry(&["core"], &[])),
            ("apps/api/crates/app/core", dir_entry(&[], &["Cargo.toml"])),
            ("apps/api/crates/ports", dir_entry(&["repo"], &[])),
            (
                "apps/api/crates/ports/repo",
                dir_entry(&[], &["Cargo.toml"]),
            ),
        ],
        vec![
            (
                "apps/api/Cargo.toml",
                "[workspace]\nmembers = [\"crates/domain/a\", \"crates/domain/b\", \"crates/domain/c\", \"crates/app/core\", \"crates/ports/repo\"]\n",
            ),
            (
                "apps/api/crates/domain/a/Cargo.toml",
                "[package]\nname = \"api-domain-a\"\n[dependencies]\napi-domain-b = { path = \"../b\" }\n",
            ),
            (
                "apps/api/crates/domain/b/Cargo.toml",
                "[package]\nname = \"api-domain-b\"\n[dependencies]\napi-domain-c = { path = \"../c\" }\n",
            ),
            (
                "apps/api/crates/domain/c/Cargo.toml",
                "[package]\nname = \"api-domain-c\"\n[dependencies]\napi-domain-a = { path = \"../a\" }\n",
            ),
            (
                "apps/api/crates/app/core/Cargo.toml",
                "[package]\nname = \"api-app-core\"\n[dependencies]\napi-ports-repo = { path = \"../../ports/repo\" }\n",
            ),
            (
                "apps/api/crates/ports/repo/Cargo.toml",
                "[package]\nname = \"api-ports-repo\"\n[dependencies]\napi-app-core = { path = \"../../app/core\" }\n",
            ),
        ],
    );

    let (cycle_layers, results) = results_for_cycles(&tree);

    assertions::assert_cycle_layers(&cycle_layers, 1, &["domain"]);
    assertions::assert_error_count(&results, "", 1);

    assertions::assert_error_file_set(&results, "", 1, &[]);
    assertions::assert_result_titles(
        &assertions::error_results(&results, ""),
        &["same-layer domain dependency cycle"],
    );
}
