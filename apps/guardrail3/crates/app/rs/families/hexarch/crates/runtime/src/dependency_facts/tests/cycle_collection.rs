use super::super::collect_for_test_tree as dependency_facts;
use super::{dir_entry, project_tree};
use guardrail3_app_rs_family_hexarch_assertions::dependency_facts as assertions;

#[test]
fn cycle_with_unlayered_member_is_not_reported_as_same_layer() {
    let tree = project_tree(
        vec![
            ("", dir_entry(&["apps"], &[])),
            ("apps", dir_entry(&["api"], &[])),
            (
                "apps/api",
                dir_entry(&["crates", "shared"], &["Cargo.toml"]),
            ),
            ("apps/api/crates", dir_entry(&["domain"], &[])),
            ("apps/api/crates/domain", dir_entry(&["core"], &[])),
            (
                "apps/api/crates/domain/core",
                dir_entry(&[], &["Cargo.toml"]),
            ),
            ("apps/api/shared", dir_entry(&["helper"], &[])),
            ("apps/api/shared/helper", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![
            (
                "apps/api/Cargo.toml",
                "[workspace]\nmembers = [\"crates/domain/core\", \"shared/helper\"]\n",
            ),
            (
                "apps/api/crates/domain/core/Cargo.toml",
                "[package]\nname = \"api-domain-core\"\n[dependencies]\napi-shared-helper = { path = \"../../../shared/helper\" }\n",
            ),
            (
                "apps/api/shared/helper/Cargo.toml",
                "[package]\nname = \"api-shared-helper\"\n[dependencies]\napi-domain-core = { path = \"../../crates/domain/core\" }\n",
            ),
        ],
    );

    let facts = dependency_facts(&tree);
    assertions::assert_no_cycles(&facts.cycles);
}
