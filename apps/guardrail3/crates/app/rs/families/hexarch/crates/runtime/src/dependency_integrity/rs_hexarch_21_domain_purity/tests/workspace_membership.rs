use super::helpers::{DomainPurityEdgeKindForTest, run_domain_purity_case};
use super::{dir_entry, project_tree};
use guardrail3_app_rs_family_hexarch_assertions::dependency_integrity::rs_hexarch_21_domain_purity as assertions;

#[test]
fn omitted_same_app_pure_layer_targets_do_not_count_as_allowed_internal_deps() {
    let tree = project_tree(
        vec![
            ("", dir_entry(&["apps"], &[])),
            ("apps", dir_entry(&["api"], &[])),
            ("apps/api", dir_entry(&["crates"], &["Cargo.toml"])),
            ("apps/api/crates", dir_entry(&["domain", "ports"], &[])),
            (
                "apps/api/crates/domain",
                dir_entry(&["core", "shared"], &[]),
            ),
            (
                "apps/api/crates/domain/core",
                dir_entry(&[], &["Cargo.toml"]),
            ),
            (
                "apps/api/crates/domain/shared",
                dir_entry(&[], &["Cargo.toml"]),
            ),
            ("apps/api/crates/ports", dir_entry(&["repo"], &[])),
            (
                "apps/api/crates/ports/repo",
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
                "[package]\nname = \"api-domain-core\"\n[dependencies]\napi-domain-shared = { path = \"../shared\" }\napi-ports-repo = { path = \"../../ports/repo\" }\n",
            ),
            (
                "apps/api/crates/domain/shared/Cargo.toml",
                "[package]\nname = \"api-domain-shared\"\n",
            ),
            (
                "apps/api/crates/ports/repo/Cargo.toml",
                "[package]\nname = \"api-ports-repo\"\n",
            ),
        ],
    );

    let results = run_domain_purity_case(
        &tree,
        "apps/api/crates/domain/core",
        DomainPurityEdgeKindForTest::Dependency,
    );

    assertions::assert_error_title_set(
        &results,
        "",
        &["domain crate `api-domain-core` depends on non-workspace pure-layer crate"],
    );
    assertions::assert_error_count(&results, "", 2);
}
