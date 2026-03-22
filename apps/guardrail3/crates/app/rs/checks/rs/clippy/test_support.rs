use std::collections::BTreeMap;
use std::path::PathBuf;

use crate::domain::project_tree::{DirEntry, ProjectTree};

pub fn dir_entry(dirs: &[&str], files: &[&str]) -> DirEntry {
    DirEntry {
        dirs: dirs.iter().map(|dir| (*dir).to_owned()).collect(),
        files: files.iter().map(|file| (*file).to_owned()).collect(),
    }
}

pub fn project_tree(
    structure: Vec<(&str, DirEntry)>,
    content: Vec<(&str, String)>,
) -> ProjectTree {
    ProjectTree {
        root: PathBuf::from("/tmp/project"),
        structure: structure
            .into_iter()
            .map(|(rel, entry)| (rel.to_owned(), entry))
            .collect::<BTreeMap<_, _>>(),
        content: content
            .into_iter()
            .map(|(rel, content)| (rel.to_owned(), content))
            .collect::<BTreeMap<_, _>>(),
    }
}

pub fn root_workspace_tree(clippy_toml: impl Into<String>) -> ProjectTree {
    project_tree(
        vec![("", dir_entry(&[], &["Cargo.toml", "clippy.toml"]))],
        vec![
            ("Cargo.toml", "[workspace]\nmembers = []".to_owned()),
            ("clippy.toml", clippy_toml.into()),
        ],
    )
}

pub fn root_workspace_tree_with_guardrail(
    clippy_toml: impl Into<String>,
    guardrail_toml: impl Into<String>,
) -> ProjectTree {
    project_tree(
        vec![(
            "",
            dir_entry(&[], &["Cargo.toml", "guardrail3.toml", "clippy.toml"]),
        )],
        vec![
            ("Cargo.toml", "[workspace]\nmembers = []".to_owned()),
            ("guardrail3.toml", guardrail_toml.into()),
            ("clippy.toml", clippy_toml.into()),
        ],
    )
}

pub fn root_coverage_tree() -> ProjectTree {
    project_tree(
        vec![
            ("", dir_entry(&["workspace", "standalone"], &["clippy.toml"])),
            ("workspace", dir_entry(&["crates"], &["Cargo.toml"])),
            ("workspace/crates", dir_entry(&["api"], &[])),
            ("workspace/crates/api", dir_entry(&[], &["Cargo.toml"])),
            ("standalone", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![
            ("workspace/Cargo.toml", "[workspace]\nmembers = [\"crates/*\"]".to_owned()),
            ("workspace/crates/api/Cargo.toml", "[package]\nname = \"api\"".to_owned()),
            ("standalone/Cargo.toml", "[package]\nname = \"standalone\"".to_owned()),
            ("clippy.toml", canonical_clippy_toml().to_owned()),
        ],
    )
}

pub fn uncovered_standalone_tree() -> ProjectTree {
    project_tree(
        vec![
            ("", dir_entry(&["standalone"], &[])),
            ("standalone", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![("standalone/Cargo.toml", "[package]\nname = \"standalone\"".to_owned())],
    )
}

pub fn nested_workspace_member_shadow_tree(file_name: &str) -> ProjectTree {
    project_tree(
        vec![
            ("", dir_entry(&["workspace"], &[])),
            ("workspace", dir_entry(&["crates"], &["Cargo.toml", "clippy.toml"])),
            ("workspace/crates", dir_entry(&["core"], &[])),
            (
                "workspace/crates/core",
                dir_entry(&[], &["Cargo.toml", file_name]),
            ),
        ],
        vec![
            ("workspace/Cargo.toml", "[workspace]\nmembers = [\"crates/*\"]".to_owned()),
            ("workspace/crates/core/Cargo.toml", "[package]\nname = \"core\"".to_owned()),
            ("workspace/clippy.toml", canonical_clippy_toml().to_owned()),
            (
                &format!("workspace/crates/core/{file_name}"),
                canonical_clippy_toml().to_owned(),
            ),
        ],
    )
}

pub fn incomplete_workspace_policy_root_tree() -> ProjectTree {
    project_tree(
        vec![
            ("", dir_entry(&["workspace"], &["clippy.toml"])),
            ("workspace", dir_entry(&["crates"], &["Cargo.toml", "clippy.toml"])),
            ("workspace/crates", dir_entry(&["core"], &[])),
            ("workspace/crates/core", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![
            ("workspace/Cargo.toml", "[workspace]\nmembers = [\"crates/*\"]".to_owned()),
            ("workspace/crates/core/Cargo.toml", "[package]\nname = \"core\"".to_owned()),
            ("clippy.toml", canonical_clippy_toml().to_owned()),
            (
                "workspace/clippy.toml",
                r#"
too-many-lines-threshold = 75
disallowed-methods = []
disallowed-types = []
disallowed-macros = []
"#
                .to_owned(),
            ),
        ],
    )
}

pub fn library_workspace_root_tree(local_clippy_toml: impl Into<String>) -> ProjectTree {
    project_tree(
        vec![
            ("", dir_entry(&["apps"], &["guardrail3.toml", "clippy.toml"])),
            ("apps", dir_entry(&["libsite"], &[])),
            (
                "apps/libsite",
                dir_entry(&["crates"], &["Cargo.toml", "clippy.toml"]),
            ),
            ("apps/libsite/crates", dir_entry(&["core"], &[])),
            ("apps/libsite/crates/core", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![
            (
                "guardrail3.toml",
                "[profile]\nname = \"service\"\n[rust.apps.libsite]\ntype = \"library\"".to_owned(),
            ),
            ("apps/libsite/Cargo.toml", "[workspace]\nmembers = [\"crates/*\"]".to_owned()),
            ("apps/libsite/crates/core/Cargo.toml", "[package]\nname = \"core\"".to_owned()),
            ("clippy.toml", canonical_clippy_toml().to_owned()),
            ("apps/libsite/clippy.toml", local_clippy_toml.into()),
        ],
    )
}

pub fn garde_disabled_root_tree(clippy_toml: impl Into<String>) -> ProjectTree {
    root_workspace_tree_with_guardrail(
        clippy_toml,
        "[profile]\nname = \"service\"\n[rust.checks]\ngarde = false",
    )
}

pub fn config_hygiene_tree() -> ProjectTree {
    root_workspace_tree(
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
"#,
    )
}

pub fn canonical_clippy_toml() -> &'static str {
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
