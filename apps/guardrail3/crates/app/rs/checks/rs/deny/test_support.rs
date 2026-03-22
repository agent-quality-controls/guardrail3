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

pub fn canonical_deny_toml_service() -> String {
    r#"
[graph]
all-features = true
no-default-features = false

[bans]
multiple-versions = "deny"
wildcards = "allow"
allow-wildcard-paths = true
highlight = "all"
deny = [
    { name = "simd-json", wrappers = [] },
    { name = "json5", wrappers = [] },
    { name = "sonic-rs", wrappers = [] },
    { name = "ureq", wrappers = [] },
    { name = "surf", wrappers = [] },
    { name = "isahc", wrappers = [] },
    { name = "log4rs", wrappers = [] },
    { name = "env_logger", wrappers = [] },
    { name = "simple_logger", wrappers = [] },
    { name = "fern", wrappers = [] },
    { name = "async-std", wrappers = [] },
    { name = "smol", wrappers = [] },
    { name = "anyhow", wrappers = [] },
    { name = "bincode", wrappers = [] },
    { name = "rmp-serde", wrappers = [] },
    { name = "actix-web", wrappers = [] },
    { name = "rocket", wrappers = [] },
    { name = "warp", wrappers = [] },
    { name = "poem", wrappers = [] },
    { name = "diesel", wrappers = [] },
    { name = "sea-orm", wrappers = [] },
    { name = "prost", wrappers = [] },
    { name = "flatbuffers", wrappers = [] },
    { name = "openssl", wrappers = [] },
    { name = "lazy_static", wrappers = [] },
]
skip = []

[[bans.features]]
name = "tokio"
deny = ["full"]
allow = ["fs", "io-util", "macros", "net", "process", "rt-multi-thread", "signal", "sync", "time"]
reason = "good enough reason text"

[licenses]
allow = ["MIT", "Apache-2.0", "BSD-3-Clause", "ISC", "Unicode-3.0", "BSD-2-Clause", "BSL-1.0", "MPL-2.0", "CDLA-Permissive-2.0", "OpenSSL", "Zlib", "CC0-1.0"]
confidence-threshold = 0.8

[licenses.private]
ignore = true

[advisories]
unmaintained = "workspace"
yanked = "warn"
ignore = []

[sources]
unknown-registry = "deny"
unknown-git = "deny"
allow-registry = ["https://github.com/rust-lang/crates.io-index"]
allow-git = []
"#
    .to_owned()
}

pub fn root_tree_with_deny(deny: &str) -> ProjectTree {
    project_tree(
        vec![("", dir_entry(&[], &["Cargo.toml", "deny.toml"]))],
        vec![
            ("Cargo.toml", "[package]\nname = \"crate\"\n".to_owned()),
            ("deny.toml", deny.to_owned()),
        ],
    )
}

pub fn root_tree_with_deny_and_guardrail(deny: &str, guardrail: &str) -> ProjectTree {
    project_tree(
        vec![("", dir_entry(&[], &["Cargo.toml", "guardrail3.toml", "deny.toml"]))],
        vec![
            ("Cargo.toml", "[package]\nname = \"crate\"\n".to_owned()),
            ("guardrail3.toml", guardrail.to_owned()),
            ("deny.toml", deny.to_owned()),
        ],
    )
}

pub fn root_coverage_tree() -> ProjectTree {
    project_tree(
        vec![
            ("", dir_entry(&["workspace", "standalone"], &["deny.toml"])),
            ("workspace", dir_entry(&["crates"], &["Cargo.toml"])),
            ("workspace/crates", dir_entry(&["api"], &[])),
            ("workspace/crates/api", dir_entry(&[], &["Cargo.toml"])),
            ("standalone", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![
            ("workspace/Cargo.toml", "[workspace]\nmembers=[\"crates/*\"]".to_owned()),
            ("workspace/crates/api/Cargo.toml", "[package]\nname=\"api\"".to_owned()),
            ("standalone/Cargo.toml", "[package]\nname=\"standalone\"".to_owned()),
            ("deny.toml", canonical_deny_toml_service()),
        ],
    )
}

pub fn uncovered_workspace_tree() -> ProjectTree {
    project_tree(
        vec![
            ("", dir_entry(&["workspace"], &[])),
            ("workspace", dir_entry(&["crates"], &["Cargo.toml"])),
            ("workspace/crates", dir_entry(&["api"], &[])),
            ("workspace/crates/api", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![
            ("workspace/Cargo.toml", "[workspace]\nmembers=[\"crates/*\"]".to_owned()),
            ("workspace/crates/api/Cargo.toml", "[package]\nname=\"api\"".to_owned()),
        ],
    )
}

pub fn nested_member_shadow_tree(file_name: &str) -> ProjectTree {
    project_tree(
        vec![
            ("", dir_entry(&["workspace"], &[])),
            ("workspace", dir_entry(&["crates"], &["Cargo.toml", "deny.toml"])),
            ("workspace/crates", dir_entry(&["core"], &[])),
            ("workspace/crates/core", dir_entry(&[], &["Cargo.toml", file_name])),
        ],
        vec![
            ("workspace/Cargo.toml", "[workspace]\nmembers=[\"crates/*\"]".to_owned()),
            ("workspace/crates/core/Cargo.toml", "[package]\nname=\"core\"".to_owned()),
            ("workspace/deny.toml", canonical_deny_toml_service()),
            (
                &format!("workspace/crates/core/{file_name}"),
                canonical_deny_toml_service(),
            ),
        ],
    )
}

pub fn same_root_conflict_tree() -> ProjectTree {
    project_tree(
        vec![
            (
                "",
                dir_entry(&[".cargo"], &["Cargo.toml", "deny.toml", ".deny.toml"]),
            ),
            (".cargo", dir_entry(&[], &["deny.toml"])),
        ],
        vec![
            ("Cargo.toml", "[package]\nname=\"crate\"\n".to_owned()),
            ("deny.toml", canonical_deny_toml_service()),
            (".deny.toml", canonical_deny_toml_service()),
            (".cargo/deny.toml", canonical_deny_toml_service()),
        ],
    )
}

pub fn library_profile_tree(deny: &str) -> ProjectTree {
    root_tree_with_deny_and_guardrail(
        deny,
        "[profile]\nname = \"service\"\n\n[rust.packages]\ntype = \"library\"\n",
    )
}
