use std::fs;

use g3rs_hexarch_source_checks::check as check_source;
use g3rs_workspace_crawl::G3RsWorkspaceIgnoreState;
use guardrail3_check_types::G3Severity;
use guardrail3_check_types::G3CheckResult;
use tempfile::tempdir;

fn source_results(root: &std::path::Path) -> Vec<G3CheckResult> {
    let crawl = super::crawl_workspace(root);
    let inputs = crate::ingest_for_source_checks(&crawl).expect("source ingest");
    inputs.iter().flat_map(check_source).collect()
}

#[test]
fn cfg_test_items_are_ignored_but_mixed_cfg_items_are_counted() {
    let root = tempdir().expect("tempdir");

    fs::create_dir_all(root.path().join("apps/demo/crates/ports/http/src")).expect("dirs");
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
"#,
    )
    .expect("member cargo");
    fs::write(
        root.path().join("apps/demo/crates/ports/http/src/lib.rs"),
        r#"
pub trait PortApi {
    fn list(&self);
}

#[cfg(test)]
pub fn ignored_helper() {}

#[cfg(any(test, feature = "debug-tools"))]
pub fn counted_helper() {}
"#,
    )
    .expect("member lib");

    let results = source_results(root.path());
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id(), "RS-HEXARCH-22");
    assert_eq!(results[0].severity(), G3Severity::Warn);
    assert!(results[0].message().contains("1 public free function"));
}

#[test]
fn path_attr_modules_and_nested_mod_rs_are_resolved() {
    let root = tempdir().expect("tempdir");

    fs::create_dir_all(root.path().join("apps/demo/crates/adapters/sql/src/wired/nested")).expect("dirs");
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
    .expect("member cargo");
    fs::write(
        root.path().join("apps/demo/crates/adapters/sql/src/lib.rs"),
        "#[path = \"wired.rs\"] pub mod outer;\n",
    )
    .expect("lib");
    fs::write(
        root.path().join("apps/demo/crates/adapters/sql/src/wired.rs"),
        "pub mod nested;\n",
    )
    .expect("wired");
    fs::write(
        root.path().join("apps/demo/crates/adapters/sql/src/wired/nested/mod.rs"),
        "pub trait AdapterBoundary {}\n",
    )
    .expect("nested");

    let results = source_results(root.path());
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id(), "RS-HEXARCH-23");
    assert_eq!(results[0].severity(), G3Severity::Error);
    assert_eq!(results[0].file(), Some("crates/adapters/sql"));
    assert!(!results[0].inventory());
    assert!(results[0].title().contains("defines public traits"));
}

#[test]
fn bin_path_and_main_fallback_are_analyzed() {
    let root = tempdir().expect("tempdir");

    fs::create_dir_all(root.path().join("apps/demo/crates/adapters/sql/src")).expect("adapter dirs");
    fs::create_dir_all(root.path().join("apps/demo/crates/ports/http/src")).expect("ports dirs");
    fs::write(
        root.path().join("apps/demo/Cargo.toml"),
        r#"
[workspace]
members = ["crates/adapters/sql", "crates/ports/http"]
"#,
    )
    .expect("workspace cargo");
    fs::write(
        root.path().join("apps/demo/crates/adapters/sql/Cargo.toml"),
        r#"
[package]
name = "adapter-sql"
version = "0.1.0"

[[bin]]
name = "sql"
path = "cli.rs"
"#,
    )
    .expect("adapter cargo");
    fs::write(
        root.path().join("apps/demo/crates/adapters/sql/cli.rs"),
        "pub trait AdapterBoundary {}\n",
    )
    .expect("cli");
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
        root.path().join("apps/demo/crates/ports/http/src/main.rs"),
        "pub fn leaked() {}\n",
    )
    .expect("main");

    let results = source_results(root.path());
    let mut ids = results.iter().map(|result| result.id().to_owned()).collect::<Vec<_>>();
    ids.sort();

    assert_eq!(ids, vec!["RS-HEXARCH-22".to_owned(), "RS-HEXARCH-23".to_owned()]);
}

#[test]
fn pub_crate_and_pub_super_adapter_traits_are_ignored() {
    let root = tempdir().expect("tempdir");

    fs::create_dir_all(root.path().join("apps/demo/crates/adapters/sql/src/nested")).expect("dirs");
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
    .expect("member cargo");
    fs::write(
        root.path().join("apps/demo/crates/adapters/sql/src/lib.rs"),
        "pub(crate) trait HiddenBoundary {}\npub mod nested;\n",
    )
    .expect("lib");
    fs::write(
        root.path().join("apps/demo/crates/adapters/sql/src/nested/mod.rs"),
        "pub(super) trait InternalBoundary {}\n",
    )
    .expect("nested");

    let results = source_results(root.path());
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id(), "RS-HEXARCH-23");
    assert!(results[0].inventory());
}

#[test]
fn inline_public_adapter_module_is_counted() {
    let root = tempdir().expect("tempdir");

    fs::create_dir_all(root.path().join("apps/demo/crates/adapters/sql/src")).expect("dirs");
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
    .expect("member cargo");
    fs::write(
        root.path().join("apps/demo/crates/adapters/sql/src/lib.rs"),
        "pub mod nested { pub trait AdapterBoundary {} }\n",
    )
    .expect("lib");

    let results = source_results(root.path());
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id(), "RS-HEXARCH-23");
    assert_eq!(results[0].severity(), G3Severity::Error);
    assert!(!results[0].inventory());
}

#[test]
fn ports_parse_failure_fails_closed_end_to_end() {
    let root = tempdir().expect("tempdir");

    fs::create_dir_all(root.path().join("apps/demo/crates/ports/http/src")).expect("dirs");
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
"#,
    )
    .expect("member cargo");
    fs::write(
        root.path().join("apps/demo/crates/ports/http/src/lib.rs"),
        "mod extra;\n",
    )
    .expect("lib");
    fs::write(
        root.path().join("apps/demo/crates/ports/http/src/extra.rs"),
        "pub fn leaked( {\n",
    )
    .expect("extra");

    let results = source_results(root.path());
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id(), "RS-HEXARCH-22");
    assert_eq!(results[0].severity(), G3Severity::Warn);
    assert_eq!(results[0].file(), Some("crates/ports/http/src/extra.rs"));
    assert!(results[0].title().contains("source analysis failed"));
}

#[test]
fn src_dir_without_lib_or_main_fails_closed() {
    let root = tempdir().expect("tempdir");

    fs::create_dir_all(root.path().join("apps/demo/crates/ports/http/src")).expect("dirs");
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
"#,
    )
    .expect("member cargo");
    fs::write(
        root.path().join("apps/demo/crates/ports/http/src/orphan.rs"),
        "pub fn leaked() {}\n",
    )
    .expect("orphan");

    let results = source_results(root.path());
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id(), "RS-HEXARCH-22");
    assert_eq!(results[0].file(), Some("crates/ports/http/src"));
    assert!(results[0].message().contains("expected src/lib.rs or src/main.rs"));
}

#[test]
fn ports_trait_impl_methods_do_not_count() {
    let root = tempdir().expect("tempdir");

    fs::create_dir_all(root.path().join("apps/demo/crates/ports/http/src")).expect("dirs");
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
"#,
    )
    .expect("member cargo");
    fs::write(
        root.path().join("apps/demo/crates/ports/http/src/lib.rs"),
        r#"
pub trait PortApi {
    fn list(&self);
}

pub struct Repo;

impl PortApi for Repo {
    fn list(&self) {}
}
"#,
    )
    .expect("lib");

    let results = source_results(root.path());
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id(), "RS-HEXARCH-22");
    assert!(results[0].inventory());
}

#[test]
fn orphan_source_files_do_not_count_toward_reachable_surface() {
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
        "pub trait PortApi { fn list(&self); }\n",
    )
    .expect("ports lib");
    fs::write(
        root.path().join("apps/demo/crates/ports/http/src/orphan.rs"),
        "pub fn leaked() {}\n",
    )
    .expect("ports orphan");
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
        "pub struct Adapter;\n",
    )
    .expect("adapter lib");
    fs::write(
        root.path().join("apps/demo/crates/adapters/sql/src/orphan.rs"),
        "pub trait BadAdapterTrait {}\n",
    )
    .expect("adapter orphan");

    let results = source_results(root.path());
    assert_eq!(results.len(), 2);
    assert!(results.iter().all(|result| result.inventory()));
}

#[test]
fn configured_missing_target_fails_closed() {
    let root = tempdir().expect("tempdir");

    fs::create_dir_all(root.path().join("apps/demo/crates/adapters/sql")).expect("dirs");
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

[lib]
path = "missing.rs"
"#,
    )
    .expect("member cargo");

    let results = source_results(root.path());
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id(), "RS-HEXARCH-23");
    assert_eq!(results[0].file(), Some("crates/adapters/sql/Cargo.toml"));
    assert!(results[0].message().contains("configured target path(s) not found"));
}

#[test]
fn unreadable_source_file_fails_closed() {
    let root = tempdir().expect("tempdir");

    fs::create_dir_all(root.path().join("apps/demo/crates/adapters/sql/src")).expect("dirs");
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
    .expect("member cargo");
    fs::write(
        root.path().join("apps/demo/crates/adapters/sql/src/lib.rs"),
        "pub mod extra;\n",
    )
    .expect("lib");
    fs::write(
        root.path().join("apps/demo/crates/adapters/sql/src/extra.rs"),
        "pub trait BadAdapterTrait {}\n",
    )
    .expect("extra");

    let mut crawl = super::crawl_workspace(root.path());
    let entry = crawl
        .entries
        .iter_mut()
        .find(|entry| entry.path.rel_path == "crates/adapters/sql/src/extra.rs")
        .expect("entry");
    entry.readable = false;

    let inputs = crate::ingest_for_source_checks(&crawl).expect("source ingest");
    let results = inputs.iter().flat_map(check_source).collect::<Vec<_>>();

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id(), "RS-HEXARCH-23");
    assert_eq!(results[0].severity(), G3Severity::Error);
    assert_eq!(results[0].file(), Some("crates/adapters/sql/src/extra.rs"));
    assert!(results[0].message().contains("file is not readable"));
}

#[test]
fn ignored_member_source_is_not_followed() {
    let root = tempdir().expect("tempdir");

    fs::create_dir_all(root.path().join("apps/demo/crates/adapters/sql/src")).expect("dirs");
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
    .expect("member cargo");
    fs::write(
        root.path().join("apps/demo/crates/adapters/sql/src/lib.rs"),
        "mod extra;\n",
    )
    .expect("lib");
    fs::write(
        root.path().join("apps/demo/crates/adapters/sql/src/extra.rs"),
        "pub trait IgnoredBoundary {}\n",
    )
    .expect("extra");

    let mut crawl = super::crawl_workspace(root.path());
    crawl
        .entries
        .iter_mut()
        .find(|entry| entry.path.rel_path == "crates/adapters/sql/src/extra.rs")
        .expect("ignored entry")
        .ignore_state = G3RsWorkspaceIgnoreState::Ignored;

    let inputs = crate::ingest_for_source_checks(&crawl).expect("source ingest");
    let results = inputs.iter().flat_map(check_source).collect::<Vec<_>>();

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id(), "RS-HEXARCH-23");
    assert!(results[0].inventory());
}
