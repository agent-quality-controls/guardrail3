use super::super::{run_domain_purity_case, DomainPurityEdgeKindForTest};
use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_21_domain_purity as assertions;
use test_support::{copy_fixture, dir_entry, project_tree, write_file};

#[test]
fn domain_and_ports_path_deps_do_not_trigger_domain_purity() {
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
                "[workspace]\nmembers = [\"crates/domain/*\", \"crates/ports/*\"]\n",
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

    assert!(
        results.is_empty(),
        "pure domain and ports path dependencies should not trigger domain purity: {results:#?}"
    );
}

#[test]
fn real_backend_ports_member_stays_allowed() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/backend/crates/domain/engine/Cargo.toml",
        "[package]\nname = \"backend-domain-engine\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\nbackend-domain-types = { path = \"../types\" }\nbackend-ports-outbound-repo = { path = \"../../ports/outbound/repo\" }\n",
    );

    let results = assertions::run_family(tmp.path());
    assert!(
        assertions::errors_by_id(&results, "RS-HEXARCH-21").is_empty(),
        "{results:#?}"
    );
}
