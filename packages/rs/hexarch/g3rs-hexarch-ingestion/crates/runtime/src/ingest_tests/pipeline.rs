use std::fs;

use g3rs_hexarch_source_checks::check as check_source;
use tempfile::tempdir;

#[test]
fn source_pipeline_reports_ports_behavior_and_adapter_public_trait() {
    let root = tempdir().expect("tempdir");

    fs::create_dir_all(root.path().join("apps/demo/crates/ports/http/src")).expect("ports dirs");
    fs::create_dir_all(root.path().join("apps/demo/crates/adapters/sql/src")).expect("adapter dirs");
    fs::write(
        root.path().join("apps/demo/Cargo.toml"),
        r#"
[workspace]
members = ["crates/ports/http", "crates/adapters/sql"]
"#,
    )
    .expect("workspace cargo");

    fs::write(
        root.path().join("apps/demo/crates/ports/http/Cargo.toml"),
        r#"
[package]
name = "ports-http"
version = "0.1.0"
"#,
    )
    .expect("ports cargo");
    fs::write(
        root.path().join("apps/demo/crates/ports/http/src/lib.rs"),
        "pub fn leaked() {}",
    )
    .expect("ports lib");
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
        "pub trait BadAdapterTrait {}",
    )
    .expect("adapter lib");

    let crawl = super::crawl_workspace(root.path());
    let inputs = crate::ingest_for_source_checks(&crawl).expect("source ingest");
    assert_eq!(inputs.len(), 2, "expected one source input per crate");

    let results = inputs
        .iter()
        .flat_map(check_source)
        .collect::<Vec<_>>();

    assert_eq!(results.len(), 2);

    let ports = results
        .iter()
        .find(|result| result.id() == "RS-HEXARCH-22")
        .expect("ports result");
    assert_eq!(ports.file(), Some("crates/ports/http"));
    assert!(!ports.inventory());
    assert!(ports.title().contains("public free functions"));

    let adapter = results
        .iter()
        .find(|result| result.id() == "RS-HEXARCH-23")
        .expect("adapter result");
    assert_eq!(adapter.file(), Some("crates/adapters/sql"));
    assert!(!adapter.inventory());
    assert!(adapter.title().contains("defines public traits"));
}
