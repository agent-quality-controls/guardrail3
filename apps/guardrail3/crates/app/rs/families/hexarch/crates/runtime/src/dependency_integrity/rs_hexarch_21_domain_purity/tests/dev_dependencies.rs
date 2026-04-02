use super::super::{DomainPurityEdgeKindForTest, run_domain_purity_case};
use super::{dir_entry, project_tree};
use guardrail3_app_rs_family_hexarch_assertions::dependency_integrity::rs_hexarch_21_domain_purity as assertions;

#[test]
fn dev_dependencies_do_not_trigger_domain_purity() {
    let tree = project_tree(
        vec![
            ("", dir_entry(&["apps"], &[])),
            ("apps", dir_entry(&["api"], &[])),
            ("apps/api", dir_entry(&["crates"], &["Cargo.toml"])),
            (
                "apps/api/crates",
                dir_entry(&["domain", "ports", "adapters"], &[]),
            ),
            ("apps/api/crates/domain", dir_entry(&["core"], &[])),
            (
                "apps/api/crates/domain/core",
                dir_entry(&[], &["Cargo.toml"]),
            ),
            ("apps/api/crates/ports", dir_entry(&["repo"], &[])),
            (
                "apps/api/crates/ports/repo",
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
                "[workspace]\nmembers = [\"crates/domain/*\", \"crates/ports/*\", \"crates/adapters/*\"]\n",
            ),
            (
                "apps/api/crates/domain/core/Cargo.toml",
                "[package]\nname = \"api-domain-core\"\n[dev-dependencies]\nserde = \"1\"\napi-ports-repo = { path = \"../../ports/repo\" }\napi-adapters-http = { path = \"../../adapters/http\" }\n",
            ),
            (
                "apps/api/crates/ports/repo/Cargo.toml",
                "[package]\nname = \"api-ports-repo\"\n",
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
        DomainPurityEdgeKindForTest::DevDependency,
    );

    assertions::assert_no_error(&results, "");
}
