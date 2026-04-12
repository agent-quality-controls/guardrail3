use std::fs;

use g3rs_hexarch_source_checks::check as check_source;
use guardrail3_check_types::G3Severity;
use tempfile::tempdir;

#[test]
fn custom_lib_path_is_used_for_ports_entrypoint() {
    let root = tempdir().expect("tempdir");

    fs::create_dir_all(root.path().join("apps/demo/crates/ports/http")).expect("ports dirs");
    fs::write(
        root.path().join("apps/demo/Cargo.toml"),
        r#"
[workspace]
members = ["crates/ports/http"]
"#,
    )
    .expect("workspace cargo");
    fs::write(
        root.path().join("apps/demo/crates/ports/http/Cargo.toml"),
        r#"
[package]
name = "ports-http"
version = "0.1.0"

[lib]
path = "mod.rs"
"#,
    )
    .expect("ports cargo");
    fs::write(
        root.path().join("apps/demo/crates/ports/http/mod.rs"),
        "pub mod api;\n",
    )
    .expect("ports mod");
    fs::write(
        root.path().join("apps/demo/crates/ports/http/api.rs"),
        "pub struct ApiAdapter;\nimpl ApiAdapter { pub fn new() -> Self { Self } }\n",
    )
    .expect("ports api");

    let crawl = super::crawl_workspace(root.path());
    let inputs = crate::ingest_for_source_checks(&crawl).expect("source ingest");
    let results = inputs.iter().flat_map(check_source).collect::<Vec<_>>();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id(), "RS-HEXARCH-SOURCE-22");
    assert_eq!(results[0].severity(), G3Severity::Warn);
    assert_eq!(results[0].file(), Some("crates/ports/http"));
    assert!(results[0].title().contains("public inherent methods"));
}

#[test]
fn private_adapter_module_public_trait_stays_clean() {
    let root = tempdir().expect("tempdir");

    fs::create_dir_all(root.path().join("apps/demo/crates/adapters/sql/src")).expect("adapter dirs");
    fs::write(
        root.path().join("apps/demo/Cargo.toml"),
        r#"
[workspace]
members = ["crates/adapters/sql"]
"#,
    )
    .expect("workspace cargo");
    fs::write(
        root.path().join("apps/demo/crates/adapters/sql/Cargo.toml"),
        r#"
[package]
name = "adapter-sql"
version = "0.1.0"
"#,
    )
    .expect("adapter cargo");
    fs::write(
        root.path().join("apps/demo/crates/adapters/sql/src/lib.rs"),
        "mod internal;\n",
    )
    .expect("adapter lib");
    fs::write(
        root.path().join("apps/demo/crates/adapters/sql/src/internal.rs"),
        "pub trait InternalBoundary {}\n",
    )
    .expect("adapter internal");

    let crawl = super::crawl_workspace(root.path());
    let inputs = crate::ingest_for_source_checks(&crawl).expect("source ingest");
    let results = inputs.iter().flat_map(check_source).collect::<Vec<_>>();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id(), "RS-HEXARCH-SOURCE-23");
    assert_eq!(results[0].severity(), G3Severity::Info);
    assert!(results[0].inventory());
}

#[test]
fn malformed_reachable_module_fails_closed_through_source_rule() {
    let root = tempdir().expect("tempdir");

    fs::create_dir_all(root.path().join("apps/demo/crates/adapters/sql/src")).expect("adapter dirs");
    fs::write(
        root.path().join("apps/demo/Cargo.toml"),
        r#"
[workspace]
members = ["crates/adapters/sql"]
"#,
    )
    .expect("workspace cargo");
    fs::write(
        root.path().join("apps/demo/crates/adapters/sql/Cargo.toml"),
        r#"
[package]
name = "adapter-sql"
version = "0.1.0"
"#,
    )
    .expect("adapter cargo");
    fs::write(
        root.path().join("apps/demo/crates/adapters/sql/src/lib.rs"),
        "mod extra;\n",
    )
    .expect("adapter lib");
    fs::write(
        root.path().join("apps/demo/crates/adapters/sql/src/extra.rs"),
        "pub trait Broken {\n",
    )
    .expect("adapter extra");

    let crawl = super::crawl_workspace(root.path());
    let inputs = crate::ingest_for_source_checks(&crawl).expect("source ingest");
    let results = inputs.iter().flat_map(check_source).collect::<Vec<_>>();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id(), "RS-HEXARCH-SOURCE-23");
    assert_eq!(results[0].severity(), G3Severity::Error);
    assert!(results[0].title().contains("source analysis failed"));
    assert_eq!(
        results[0].file(),
        Some("crates/adapters/sql/src/extra.rs")
    );
}
