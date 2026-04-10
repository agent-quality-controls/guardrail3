use std::fs;

use g3rs_arch_config_checks::check as check_config;
use g3rs_arch_source_checks::check as check_source;
use g3rs_workspace_crawl::crawl;
use tempfile::tempdir;

#[test]
fn source_pipeline_reports_lib_body_logic_and_path_attr() {
    let root = tempdir().expect("tempdir");

    fs::write(
        root.path().join("Cargo.toml"),
        r#"
[workspace]
members = ["crate_a"]
"#,
    )
    .expect("root cargo");
    fs::create_dir_all(root.path().join("crate_a/src/nested")).expect("crate dirs");
    fs::write(
        root.path().join("crate_a/Cargo.toml"),
        r#"
[package]
name = "crate_a"
version = "0.1.0"

[features]
all = ["api"]
default = ["all"]
api = []
"#,
    )
    .expect("crate cargo");
    fs::write(
        root.path().join("crate_a/src/lib.rs"),
        r#"
pub fn leaked() {}

#[path = "nested/custom.rs"]
pub mod nested;
"#,
    )
    .expect("lib");
    fs::write(root.path().join("crate_a/src/nested/custom.rs"), "pub struct Value;")
        .expect("custom");

    let crawl = crawl(root.path()).expect("crawl");
    let inputs = crate::ingest_for_source_checks(&crawl).expect("source ingest");
    let results = check_source(&inputs[0]);

    assert!(results.iter().any(|result| result.id() == "RS-ARCH-02"));
    assert!(results.iter().any(|result| result.id() == "RS-ARCH-09"));
}

#[test]
fn config_pipeline_reports_boundary_crossing_and_missing_shared_flag() {
    let root = tempdir().expect("tempdir");

    fs::write(
        root.path().join("Cargo.toml"),
        r#"
[workspace]
members = ["pkg", "pkg/crates/inner", "other"]
"#,
    )
    .expect("root cargo");

    fs::create_dir_all(root.path().join("pkg/src")).expect("pkg src");
    fs::create_dir_all(root.path().join("pkg/crates/inner/src")).expect("inner src");
    fs::create_dir_all(root.path().join("other/src")).expect("other src");

    fs::write(
        root.path().join("pkg/Cargo.toml"),
        r#"
[package]
name = "pkg"
version = "0.1.0"

[dependencies]
inner = { path = "crates/inner" }
other = { path = "../other" }
"#,
    )
    .expect("pkg cargo");
    fs::write(root.path().join("pkg/src/lib.rs"), "pub mod api;").expect("pkg lib");

    fs::write(
        root.path().join("pkg/crates/inner/Cargo.toml"),
        r#"
[package]
name = "inner"
version = "0.1.0"
"#,
    )
    .expect("inner cargo");
    fs::write(root.path().join("pkg/crates/inner/src/lib.rs"), "pub struct Inner;")
        .expect("inner lib");

    fs::write(
        root.path().join("other/Cargo.toml"),
        r#"
[package]
name = "other"
version = "0.1.0"
"#,
    )
    .expect("other cargo");
    fs::write(root.path().join("other/src/lib.rs"), "pub struct Other;").expect("other lib");

    let crawl = crawl(root.path()).expect("crawl");
    let inputs = crate::ingest_for_config_checks(&crawl).expect("config ingest");
    let results = check_config(&inputs[0]);

    assert!(results.iter().any(|result| result.id() == "RS-ARCH-05"));
    assert!(results.iter().any(|result| result.id() == "RS-ARCH-06"));
}
