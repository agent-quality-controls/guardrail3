use std::fs;

use g3rs_hexarch_source_checks::check as check_source;
use g3rs_workspace_crawl::crawl;
use tempfile::tempdir;

#[test]
fn source_pipeline_reports_ports_behavior_and_adapter_public_trait() {
    let root = tempdir().expect("tempdir");

    fs::create_dir_all(root.path().join("apps/demo/crates/ports/http/src")).expect("ports dirs");
    fs::create_dir_all(root.path().join("apps/demo/crates/adapters/sql/src")).expect("adapter dirs");

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

    let crawl = crawl(root.path()).expect("crawl");
    let inputs = crate::ingest_for_source_checks(&crawl).expect("source ingest");
    assert_eq!(inputs.len(), 2, "expected one source input per crate");

    let results = inputs
        .iter()
        .flat_map(check_source)
        .collect::<Vec<_>>();
    let mut ids = results.iter().map(|result| result.id().to_owned()).collect::<Vec<_>>();
    ids.sort();

    assert_eq!(ids, vec!["RS-HEXARCH-22".to_owned(), "RS-HEXARCH-23".to_owned()]);
}
