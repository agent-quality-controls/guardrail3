use super::super::{DomainPurityEdgeKindForTest, run_domain_purity_case};
use super::{dir_entry, project_tree};
use guardrail3_app_rs_family_hexarch_assertions::dependency_integrity::rs_hexarch_21_domain_purity as assertions;

#[test]
fn target_dependencies_are_in_scope_for_domain_purity() {
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
                "[package]\nname = \"api-domain-core\"\n[target.'cfg(unix)'.dependencies]\ntokio = \"1\"\napi-adapters-http = { path = \"../../adapters/http\" }\n",
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
        DomainPurityEdgeKindForTest::TargetDependency,
    );

    assertions::assert_error_title_set(
        &results,
        "",
        &[
            "domain crate `api-domain-core` depends on disallowed external crate `tokio`",
            "domain crate `api-domain-core` depends on non-pure layer",
        ],
    );
}

#[test]
fn target_build_dependencies_are_in_scope_for_domain_purity() {
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
                "[package]\nname = \"api-domain-core\"\n[target.'cfg(unix)'.build-dependencies]\nprost = \"0.12\"\napi-adapters-http = { path = \"../../adapters/http\" }\n",
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
        DomainPurityEdgeKindForTest::TargetBuildDependency,
    );

    assertions::assert_error_title_set(
        &results,
        "",
        &[
            "domain crate `api-domain-core` depends on disallowed external crate `prost`",
            "domain crate `api-domain-core` depends on non-pure layer",
        ],
    );
}
