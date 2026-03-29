use super::super::run_tree as run_family;
use super::super::{StubToolChecker, dir_entry, project_tree, temp_root};
use super::super::{edge_facts, edge_input};
use super::super::check;

#[test]
fn does_not_error_when_local_publishable_version_is_compatible() {
    let facts = edge_facts();
    let input = edge_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(results.is_empty());
}

#[test]
fn does_not_error_for_non_publishable_or_non_path_edges() {
    let mut non_publishable = edge_facts();
    non_publishable.dep_publishable = false;
    let non_publishable_input = edge_input(&non_publishable);
    let mut non_publishable_results = Vec::new();

    check(&non_publishable_input, &mut non_publishable_results);

    assert!(non_publishable_results.is_empty());

    let mut non_path = edge_facts();
    non_path.has_path = false;
    non_path.version_satisfied = Some(false);
    let non_path_input = edge_input(&non_path);
    let mut non_path_results = Vec::new();

    check(&non_path_input, &mut non_path_results);

    assert!(non_path_results.is_empty());

    let mut missing_req = edge_facts();
    missing_req.version_req = None;
    missing_req.version_satisfied = Some(false);
    let missing_req_input = edge_input(&missing_req);
    let mut missing_req_results = Vec::new();

    check(&missing_req_input, &mut missing_req_results);

    assert!(missing_req_results.is_empty());

    let mut unknown_compat = edge_facts();
    unknown_compat.version_satisfied = None;
    let unknown_compat_input = edge_input(&unknown_compat);
    let mut unknown_compat_results = Vec::new();

    check(&unknown_compat_input, &mut unknown_compat_results);

    assert!(unknown_compat_results.is_empty());
}

#[test]
fn errors_on_renamed_and_workspace_versioned_local_mismatches() {
    let root = temp_root("release-renamed-version-mismatch");
    let tree = project_tree(
        vec![
            ("", dir_entry(&["crates"], &["Cargo.toml"])),
            ("crates", dir_entry(&["consumer", "api"], &[])),
            ("crates/consumer", dir_entry(&[], &["Cargo.toml"])),
            ("crates/api", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![
            (
                "Cargo.toml",
                r#"
[workspace]
members = ["crates/consumer", "crates/api"]
resolver = "2"

[workspace.package]
version = "1.2.3"
"#,
            ),
            (
                "crates/consumer/Cargo.toml",
                r#"
[package]
name = "consumer"
version = "0.1.0"
edition = "2024"
description = "consumer"
license = "MIT"
repository = "https://example.com/consumer"

[dependencies]
api_v2 = { package = "api", path = "../api", version = "^2.0.0" }
"#,
            ),
            (
                "crates/api/Cargo.toml",
                r#"
[package]
name = "api"
version.workspace = true
edition = "2024"
description = "api"
license = "MIT"
repository = "https://example.com/api"
"#,
            ),
        ],
        root,
    );
    let results = run_family(&tree, &StubToolChecker::new(true), false);

    assert!(results.iter().any(|result| {
        result.id == "RS-PUB-11"
            && result.message.contains("api_v2")
            && result.message.contains("package `api`")
            && result.message.contains("`[dependencies]`")
            && result.file.as_deref() == Some("crates/consumer/Cargo.toml")
    }));
}
