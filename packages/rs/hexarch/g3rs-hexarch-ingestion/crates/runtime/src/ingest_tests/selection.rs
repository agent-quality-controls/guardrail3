use std::fs;

use g3rs_hexarch_source_checks::check as check_source;
use g3rs_workspace_crawl::G3RsWorkspaceCrawl;
use guardrail3_check_types::G3CheckResult;
use tempfile::tempdir;

fn run_source(root: &std::path::Path) -> Vec<G3CheckResult> {
    let crawl = super::crawl_workspace(root);
    run_source_from_crawl(&crawl)
}

fn run_source_from_crawl(crawl: &G3RsWorkspaceCrawl) -> Vec<G3CheckResult> {
    let inputs = crate::ingest_for_source_checks(crawl).expect("source ingest");
    inputs.iter().flat_map(check_source).collect()
}

#[test]
fn pointed_workspace_root_is_used_for_member_discovery() {
    let root = tempdir().expect("tempdir");

    fs::create_dir_all(root.path().join("crates/ports/http/src")).expect("dirs");
    fs::write(
        root.path().join("Cargo.toml"),
        r#"
[workspace]
members = ["crates/ports/http"]
"#,
    )
    .expect("workspace cargo");
    fs::write(
        root.path().join("crates/ports/http/Cargo.toml"),
        r#"
[package]
name = "ports-http"
version = "0.1.0"
"#,
    )
    .expect("member cargo");
    fs::write(root.path().join("crates/ports/http/src/lib.rs"), "pub fn leaked() {}\n")
        .expect("member lib");

    let crawl = super::crawl_workspace(root.path());
    let inputs = crate::ingest_for_source_checks(&crawl).expect("source ingest");

    assert_eq!(inputs.len(), 1);
    assert_eq!(inputs[0].crate_facts.rel_dir, "crates/ports/http");
}

#[test]
fn pointed_workspace_ignores_nested_foreign_workspaces() {
    let root = tempdir().expect("tempdir");

    fs::create_dir_all(root.path().join("crates/ports/http/src")).expect("real dirs");
    fs::create_dir_all(root.path().join("apps/foreign/crates/adapters/sql/src")).expect("foreign dirs");
    fs::write(
        root.path().join("Cargo.toml"),
        r#"
[workspace]
members = ["crates/ports/http"]
"#,
    )
    .expect("workspace cargo");
    fs::write(
        root.path().join("crates/ports/http/Cargo.toml"),
        r#"
[package]
name = "ports-http"
version = "0.1.0"
"#,
    )
    .expect("real cargo");
    fs::write(root.path().join("crates/ports/http/src/lib.rs"), "pub fn leaked() {}\n")
        .expect("real lib");
    fs::write(
        root.path().join("apps/foreign/Cargo.toml"),
        r#"
[workspace]
members = ["crates/adapters/sql"]
"#,
    )
    .expect("foreign workspace cargo");
    fs::write(
        root.path().join("apps/foreign/crates/adapters/sql/Cargo.toml"),
        r#"
[package]
name = "adapter-sql"
version = "0.1.0"
"#,
    )
    .expect("foreign cargo");
    fs::write(
        root.path().join("apps/foreign/crates/adapters/sql/src/lib.rs"),
        "pub trait BadAdapterTrait {}\n",
    )
    .expect("foreign lib");

    let crawl = super::crawl_workspace(root.path());
    let inputs = crate::ingest_for_source_checks(&crawl).expect("source ingest");
    let results = run_source_from_crawl(&crawl);

    assert_eq!(inputs.len(), 1);
    assert_eq!(inputs[0].crate_facts.rel_dir, "crates/ports/http");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id(), "RS-HEXARCH-22");
    assert_eq!(results[0].file(), Some("crates/ports/http"));
}
#[test]
fn app_root_workspace_manifest_is_not_treated_as_source_crate() {
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
        "pub fn leaked() {}\n",
    )
    .expect("member lib");

    let crawl = super::crawl_workspace(root.path());
    let inputs = crate::ingest_for_source_checks(&crawl).expect("source ingest");

    assert_eq!(inputs.len(), 1);
    assert_eq!(inputs[0].crate_facts.rel_dir, "crates/ports/http");
}

#[test]
fn invalid_member_manifest_fails_closed_per_crate_without_aborting_lane() {
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
        "pub fn leaked() {}\n",
    )
    .expect("ports lib");
    fs::write(
        root.path().join("apps/demo/crates/adapters/sql/Cargo.toml"),
        "[package\nname = \"adapter-sql\"\n",
    )
    .expect("adapter bad cargo");

    let results = run_source(root.path());
    let mut ids = results.iter().map(|result| result.id().to_owned()).collect::<Vec<_>>();
    ids.sort();

    assert_eq!(ids, vec!["RS-HEXARCH-22".to_owned(), "RS-HEXARCH-23".to_owned()]);
    let adapter = results
        .iter()
        .find(|result| result.id() == "RS-HEXARCH-23")
        .expect("adapter result");
    assert_eq!(adapter.file(), Some("crates/adapters/sql/Cargo.toml"));
    assert!(adapter.title().contains("source analysis failed"));
}

#[test]
fn crate_without_source_entrypoint_is_skipped() {
    let root = tempdir().expect("tempdir");

    fs::create_dir_all(root.path().join("apps/demo/crates/ports/http")).expect("dirs");
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

    let crawl = super::crawl_workspace(root.path());
    let inputs = crate::ingest_for_source_checks(&crawl).expect("source ingest");

    assert!(inputs.is_empty());
}

#[test]
fn fixture_members_are_excluded_from_source_lane() {
    let root = tempdir().expect("tempdir");

    fs::create_dir_all(root.path().join("apps/demo/tests/fixtures/adapters/sql/src")).expect("fixture dirs");
    fs::create_dir_all(root.path().join("apps/demo/crates/ports/http/src")).expect("real dirs");
    fs::write(
        root.path().join("apps/demo/Cargo.toml"),
        r#"
[workspace]
members = ["tests/fixtures/adapters/sql", "crates/ports/http"]
"#,
    )
    .expect("workspace cargo");
    fs::write(
        root.path().join("apps/demo/tests/fixtures/adapters/sql/Cargo.toml"),
        r#"
[package]
name = "fixture-adapter-sql"
version = "0.1.0"
"#,
    )
    .expect("fixture cargo");
    fs::write(
        root.path().join("apps/demo/tests/fixtures/adapters/sql/src/lib.rs"),
        "pub trait BadAdapterTrait {}\n",
    )
    .expect("fixture lib");
    fs::write(
        root.path().join("apps/demo/crates/ports/http/Cargo.toml"),
        r#"
[package]
name = "ports-http"
version = "0.1.0"
"#,
    )
    .expect("real cargo");
    fs::write(
        root.path().join("apps/demo/crates/ports/http/src/lib.rs"),
        "pub fn leaked() {}\n",
    )
    .expect("real lib");

    let crawl = super::crawl_workspace(root.path());
    let inputs = crate::ingest_for_source_checks(&crawl).expect("source ingest");

    assert_eq!(inputs.len(), 1);
    assert_eq!(inputs[0].crate_facts.rel_dir, "crates/ports/http");
}

#[test]
fn invalid_app_root_workspace_manifest_does_not_abort_other_apps() {
    let root = tempdir().expect("tempdir");

    fs::create_dir_all(root.path().join("apps/bad")).expect("bad dirs");
    fs::create_dir_all(root.path().join("apps/good/crates/ports/http/src")).expect("good dirs");
    fs::write(root.path().join("apps/bad/Cargo.toml"), "[workspace\n").expect("bad workspace");
    fs::write(
        root.path().join("apps/good/Cargo.toml"),
        r#"
[workspace]
members = ["crates/ports/http"]
"#,
    )
    .expect("good workspace");
    fs::write(
        root.path().join("apps/good/crates/ports/http/Cargo.toml"),
        r#"
[package]
name = "ports-http"
version = "0.1.0"
"#,
    )
    .expect("member cargo");
    fs::write(
        root.path().join("apps/good/crates/ports/http/src/lib.rs"),
        "pub fn leaked() {}\n",
    )
    .expect("member lib");

    let crawl = g3rs_workspace_crawl::crawl(&root.path().join("apps/good")).expect("crawl");
    let results = run_source_from_crawl(&crawl);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id(), "RS-HEXARCH-22");
}

#[test]
fn glob_workspace_members_resolve_to_real_crates_only() {
    let root = tempdir().expect("tempdir");

    fs::create_dir_all(root.path().join("apps/demo/crates/ports/http/src")).expect("ports dirs");
    fs::create_dir_all(root.path().join("apps/demo/crates/ports/empty")).expect("empty dirs");
    fs::write(
        root.path().join("apps/demo/Cargo.toml"),
        r#"
[workspace]
members = ["crates/ports/*"]
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
        "pub fn leaked() {}\n",
    )
    .expect("member lib");

    let crawl = super::crawl_workspace(root.path());
    let inputs = crate::ingest_for_source_checks(&crawl).expect("source ingest");

    assert_eq!(inputs.len(), 1);
    assert_eq!(inputs[0].crate_facts.rel_dir, "crates/ports/http");
}

#[test]
fn unreadable_member_manifest_fails_closed_per_crate_without_aborting_lane() {
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
        "pub fn leaked() {}\n",
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

    let mut crawl = super::crawl_workspace(root.path());
    crawl
        .entries
        .iter_mut()
        .find(|entry| entry.path.rel_path == "crates/adapters/sql/Cargo.toml")
        .expect("entry")
        .readable = false;

    let results = run_source_from_crawl(&crawl);
    let adapter = results
        .iter()
        .find(|result| result.id() == "RS-HEXARCH-23")
        .expect("adapter result");

    assert_eq!(results.len(), 2);
    assert_eq!(adapter.file(), Some("crates/adapters/sql/Cargo.toml"));
    assert!(adapter.title().contains("source analysis failed"));
    assert!(adapter.message().contains("file is not readable"));
}

#[test]
fn workspace_exclude_keeps_excluded_members_out_of_source_lane() {
    let root = tempdir().expect("tempdir");

    fs::create_dir_all(root.path().join("crates/ports/http/src")).expect("real dirs");
    fs::create_dir_all(root.path().join("crates/adapters/sql/src")).expect("excluded dirs");
    fs::write(
        root.path().join("Cargo.toml"),
        r#"
[workspace]
members = ["crates/ports/http", "crates/adapters/sql"]
exclude = ["crates/adapters/sql"]
"#,
    )
    .expect("workspace cargo");
    fs::write(
        root.path().join("crates/ports/http/Cargo.toml"),
        r#"
[package]
name = "ports-http"
version = "0.1.0"
"#,
    )
    .expect("real cargo");
    fs::write(root.path().join("crates/ports/http/src/lib.rs"), "pub fn leaked() {}\n")
        .expect("real lib");
    fs::write(
        root.path().join("crates/adapters/sql/Cargo.toml"),
        r#"
[package]
name = "adapter-sql"
version = "0.1.0"
"#,
    )
    .expect("excluded cargo");
    fs::write(
        root.path().join("crates/adapters/sql/src/lib.rs"),
        "pub trait BadAdapterTrait {}\n",
    )
    .expect("excluded lib");

    let crawl = super::crawl_workspace(root.path());
    let inputs = crate::ingest_for_source_checks(&crawl).expect("source ingest");
    let results = run_source_from_crawl(&crawl);

    assert_eq!(inputs.len(), 1);
    assert_eq!(inputs[0].crate_facts.rel_dir, "crates/ports/http");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id(), "RS-HEXARCH-22");
    assert_eq!(results[0].file(), Some("crates/ports/http"));
}

#[test]
fn unresolved_workspace_member_pattern_fails_closed() {
    let root = tempdir().expect("tempdir");

    fs::write(
        root.path().join("Cargo.toml"),
        r#"
[workspace]
members = ["crates/ports/*"]
"#,
    )
    .expect("workspace cargo");

    let crawl = super::crawl_workspace(root.path());
    let err = crate::ingest_for_source_checks(&crawl).expect_err("missing member pattern should fail");

    assert!(err.to_string().contains("did not resolve"));
}
