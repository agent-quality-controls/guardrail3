use std::collections::BTreeMap;
use std::path::PathBuf;

use crate::domain::project_tree::{DirEntry, ProjectTree};

use super::check;

#[test]
fn root_clippy_covers_workspace_and_standalone_package() {
    let tree = ProjectTree {
        root: PathBuf::from("/tmp/project"),
        structure: BTreeMap::from([
            (
                "".to_owned(),
                DirEntry {
                    dirs: vec!["workspace".to_owned(), "standalone".to_owned()],
                    files: vec!["clippy.toml".to_owned()],
                },
            ),
            (
                "workspace".to_owned(),
                DirEntry {
                    dirs: vec!["crates".to_owned()],
                    files: vec!["Cargo.toml".to_owned()],
                },
            ),
            (
                "workspace/crates".to_owned(),
                DirEntry {
                    dirs: vec!["api".to_owned()],
                    files: vec![],
                },
            ),
            (
                "workspace/crates/api".to_owned(),
                DirEntry {
                    dirs: vec![],
                    files: vec!["Cargo.toml".to_owned()],
                },
            ),
            (
                "standalone".to_owned(),
                DirEntry {
                    dirs: vec![],
                    files: vec!["Cargo.toml".to_owned()],
                },
            ),
        ]),
        content: BTreeMap::from([
            (
                "workspace/Cargo.toml".to_owned(),
                "[workspace]\nmembers = [\"crates/*\"]".to_owned(),
            ),
            (
                "workspace/crates/api/Cargo.toml".to_owned(),
                "[package]\nname = \"api\"".to_owned(),
            ),
            (
                "standalone/Cargo.toml".to_owned(),
                "[package]\nname = \"standalone\"".to_owned(),
            ),
            ("clippy.toml".to_owned(), canonical_clippy_toml().to_owned()),
        ]),
    };

    let results = check(&tree);
    assert!(results.iter().any(|r| {
        r.id == "RS-CLIPPY-01" && r.inventory && r.message.contains("workspace root `workspace`")
    }));
    assert!(results.iter().any(|r| {
        r.id == "RS-CLIPPY-01"
            && r.inventory
            && r.message.contains("standalone package root `standalone`")
    }));
}

#[test]
fn uncovered_standalone_package_is_an_error() {
    let tree = ProjectTree {
        root: PathBuf::from("/tmp/project"),
        structure: BTreeMap::from([
            (
                "".to_owned(),
                DirEntry {
                    dirs: vec!["standalone".to_owned()],
                    files: vec![],
                },
            ),
            (
                "standalone".to_owned(),
                DirEntry {
                    dirs: vec![],
                    files: vec!["Cargo.toml".to_owned()],
                },
            ),
        ]),
        content: BTreeMap::from([(
            "standalone/Cargo.toml".to_owned(),
            "[package]\nname = \"standalone\"".to_owned(),
        )]),
    };

    let results = check(&tree);
    assert!(results.iter().any(|r| {
        r.id == "RS-CLIPPY-01"
            && !r.inventory
            && r.message.contains("standalone package root `standalone`")
    }));
}

#[test]
fn nested_member_clippy_is_forbidden() {
    let tree = ProjectTree {
        root: PathBuf::from("/tmp/project"),
        structure: BTreeMap::from([
            (
                "".to_owned(),
                DirEntry {
                    dirs: vec!["workspace".to_owned()],
                    files: vec![],
                },
            ),
            (
                "workspace".to_owned(),
                DirEntry {
                    dirs: vec!["crates".to_owned()],
                    files: vec!["Cargo.toml".to_owned(), "clippy.toml".to_owned()],
                },
            ),
            (
                "workspace/crates".to_owned(),
                DirEntry {
                    dirs: vec!["core".to_owned()],
                    files: vec![],
                },
            ),
            (
                "workspace/crates/core".to_owned(),
                DirEntry {
                    dirs: vec![],
                    files: vec!["Cargo.toml".to_owned(), "clippy.toml".to_owned()],
                },
            ),
        ]),
        content: BTreeMap::from([
            (
                "workspace/Cargo.toml".to_owned(),
                "[workspace]\nmembers = [\"crates/*\"]".to_owned(),
            ),
            (
                "workspace/crates/core/Cargo.toml".to_owned(),
                "[package]\nname = \"core\"".to_owned(),
            ),
            (
                "workspace/clippy.toml".to_owned(),
                canonical_clippy_toml().to_owned(),
            ),
            (
                "workspace/crates/core/clippy.toml".to_owned(),
                canonical_clippy_toml().to_owned(),
            ),
        ]),
    };

    let results = check(&tree);
    assert!(results.iter().any(|r| {
        r.id == "RS-CLIPPY-12"
            && !r.inventory
            && r.file.as_deref() == Some("workspace/crates/core/clippy.toml")
    }));
}

#[test]
fn allowed_workspace_root_must_be_self_contained() {
    let tree = ProjectTree {
        root: PathBuf::from("/tmp/project"),
        structure: BTreeMap::from([
            (
                "".to_owned(),
                DirEntry {
                    dirs: vec!["workspace".to_owned()],
                    files: vec!["clippy.toml".to_owned()],
                },
            ),
            (
                "workspace".to_owned(),
                DirEntry {
                    dirs: vec!["crates".to_owned()],
                    files: vec!["Cargo.toml".to_owned(), "clippy.toml".to_owned()],
                },
            ),
            (
                "workspace/crates".to_owned(),
                DirEntry {
                    dirs: vec!["core".to_owned()],
                    files: vec![],
                },
            ),
            (
                "workspace/crates/core".to_owned(),
                DirEntry {
                    dirs: vec![],
                    files: vec!["Cargo.toml".to_owned()],
                },
            ),
        ]),
        content: BTreeMap::from([
            (
                "workspace/Cargo.toml".to_owned(),
                "[workspace]\nmembers = [\"crates/*\"]".to_owned(),
            ),
            (
                "workspace/crates/core/Cargo.toml".to_owned(),
                "[package]\nname = \"core\"".to_owned(),
            ),
            ("clippy.toml".to_owned(), canonical_clippy_toml().to_owned()),
            (
                "workspace/clippy.toml".to_owned(),
                r#"
too-many-lines-threshold = 75
disallowed-methods = []
disallowed-types = []
disallowed-macros = []
"#
                .to_owned(),
            ),
        ]),
    };

    let results = check(&tree);
    assert!(results.iter().any(|r| {
        r.id == "RS-CLIPPY-13"
            && !r.inventory
            && r.file.as_deref() == Some("workspace/clippy.toml")
            && r.message.contains("thresholds")
            && r.message.contains("avoid-breaking-exported-api")
    }));
}

#[test]
fn config_hygiene_and_macro_baseline_apply_to_allowed_policy_roots() {
    let tree = ProjectTree {
        root: PathBuf::from("/tmp/project"),
        structure: BTreeMap::from([(
            "".to_owned(),
            DirEntry {
                dirs: vec![],
                files: vec!["Cargo.toml".to_owned(), "clippy.toml".to_owned()],
            },
        )]),
        content: BTreeMap::from([
            (
                "Cargo.toml".to_owned(),
                "[workspace]\nmembers = []".to_owned(),
            ),
            (
                "clippy.toml".to_owned(),
                r#"
too-many-lines-threshold = 75
cognitive-complexity-threshold = 15
too-many-arguments-threshold = 7
type-complexity-threshold = 75
max-struct-bools = 3
max-fn-params-bools = 3
excessive-nesting-threshold = 4
avoid-breaking-exported-api = true
allow-dbg-in-tests = true
disalowed-methods = []

disallowed-methods = [
    { path = "std::env::var", reason = "todo" },
    { path = "std::env::var", reason = "good enough reason text" },
]

disallowed-types = [
    { path = "std::collections::HashMap", reason = "placeholder" },
]

disallowed-macros = [
    { path = "println", reason = "todo" },
    { path = "println", reason = "duplicate macro reason" },
]
"#
                .to_owned(),
            ),
        ]),
    };

    let results = check(&tree);
    assert!(results.iter().any(|r| r.id == "RS-CLIPPY-15"));
    assert!(results.iter().any(|r| r.id == "RS-CLIPPY-16"));
    assert!(results.iter().any(|r| r.id == "RS-CLIPPY-17"));
    assert!(results.iter().any(|r| r.id == "RS-CLIPPY-18"));
    assert!(results.iter().any(|r| r.id == "RS-CLIPPY-19"));
    assert!(
        results
            .iter()
            .any(|r| { r.id == "RS-CLIPPY-20" && !r.inventory && r.message.contains("eprintln!") })
    );
}

fn canonical_clippy_toml() -> &'static str {
    r#"
too-many-lines-threshold = 75
cognitive-complexity-threshold = 15
too-many-arguments-threshold = 7
type-complexity-threshold = 75
max-struct-bools = 3
max-fn-params-bools = 3
excessive-nesting-threshold = 4
avoid-breaking-exported-api = false
allow-dbg-in-tests = false
allow-print-in-tests = false

disallowed-methods = [
    { path = "std::env::var", reason = "good enough reason text" },
    { path = "std::env::var_os", reason = "good enough reason text" },
    { path = "std::env::vars", reason = "good enough reason text" },
    { path = "std::env::set_var", reason = "good enough reason text" },
    { path = "std::env::remove_var", reason = "good enough reason text" },
    { path = "std::process::exit", reason = "good enough reason text" },
    { path = "std::process::abort", reason = "good enough reason text" },
    { path = "std::process::Command::new", reason = "good enough reason text" },
    { path = "std::thread::sleep", reason = "good enough reason text" },
    { path = "std::fs::read_to_string", reason = "good enough reason text" },
    { path = "std::fs::read", reason = "good enough reason text" },
    { path = "std::fs::read_dir", reason = "good enough reason text" },
    { path = "std::fs::read_link", reason = "good enough reason text" },
    { path = "std::fs::write", reason = "good enough reason text" },
    { path = "std::fs::remove_file", reason = "good enough reason text" },
    { path = "std::fs::remove_dir_all", reason = "good enough reason text" },
    { path = "std::fs::create_dir_all", reason = "good enough reason text" },
    { path = "std::fs::rename", reason = "good enough reason text" },
    { path = "std::fs::copy", reason = "good enough reason text" },
    { path = "std::fs::metadata", reason = "good enough reason text" },
    { path = "std::fs::symlink_metadata", reason = "good enough reason text" },
    { path = "std::fs::canonicalize", reason = "good enough reason text" },
    { path = "std::fs::set_permissions", reason = "good enough reason text" },
    { path = "std::fs::hard_link", reason = "good enough reason text" },
    { path = "reqwest::Client::new", reason = "good enough reason text" },
    { path = "reqwest::Client::builder", reason = "good enough reason text" },
    { path = "serde_json::from_str", reason = "good enough reason text" },
    { path = "serde_json::from_slice", reason = "good enough reason text" },
    { path = "serde_json::from_value", reason = "good enough reason text" },
    { path = "serde_json::from_reader", reason = "good enough reason text" },
    { path = "reqwest::Response::json", reason = "good enough reason text" },
    { path = "toml::from_str", reason = "good enough reason text" },
    { path = "serde_yaml::from_str", reason = "good enough reason text" },
    { path = "serde_yaml::from_reader", reason = "good enough reason text" },
]

disallowed-types = [
    { path = "std::collections::HashMap", reason = "good enough reason text" },
    { path = "std::collections::HashSet", reason = "good enough reason text" },
    { path = "std::sync::Mutex", reason = "good enough reason text" },
    { path = "std::sync::RwLock", reason = "good enough reason text" },
    { path = "std::fs::File", reason = "good enough reason text" },
    { path = "axum::extract::Json", reason = "good enough reason text" },
    { path = "axum::Json", reason = "good enough reason text" },
    { path = "axum::extract::Query", reason = "good enough reason text" },
    { path = "axum::extract::Form", reason = "good enough reason text" },
    { path = "std::any::Any", reason = "good enough reason text" },
]

disallowed-macros = [
    { path = "println", reason = "good enough reason text" },
    { path = "eprintln", reason = "good enough reason text" },
    { path = "dbg", reason = "good enough reason text" },
    { path = "todo", reason = "good enough reason text" },
    { path = "unimplemented", reason = "good enough reason text" },
]
"#
}
