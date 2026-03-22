use std::collections::BTreeMap;
use std::path::PathBuf;

use super::facts::{DenyConfigFacts, DenyFacts, collect};
use super::inputs::{
    CoveredRustUnitInput, ForbiddenDenyConfigInput, SameRootConflictInput, UncoveredRustUnitInput,
};
use crate::domain::project_tree::{DirEntry, ProjectTree};

pub fn dir_entry(dirs: &[&str], files: &[&str]) -> DirEntry {
    DirEntry {
        dirs: dirs.iter().map(|dir| (*dir).to_owned()).collect(),
        files: files.iter().map(|file| (*file).to_owned()).collect(),
    }
}

pub fn project_tree(structure: Vec<(&str, DirEntry)>, content: Vec<(&str, String)>) -> ProjectTree {
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
            (
                "workspace/Cargo.toml",
                "[workspace]\nmembers=[\"crates/*\"]".to_owned(),
            ),
            (
                "workspace/crates/api/Cargo.toml",
                "[package]\nname=\"api\"".to_owned(),
            ),
            (
                "standalone/Cargo.toml",
                "[package]\nname=\"standalone\"".to_owned(),
            ),
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
            (
                "workspace/Cargo.toml",
                "[workspace]\nmembers=[\"crates/*\"]".to_owned(),
            ),
            (
                "workspace/crates/api/Cargo.toml",
                "[package]\nname=\"api\"".to_owned(),
            ),
        ],
    )
}

pub fn nested_member_shadow_tree(file_name: &str) -> ProjectTree {
    project_tree(
        vec![
            ("", dir_entry(&["workspace"], &[])),
            (
                "workspace",
                dir_entry(&["crates"], &["Cargo.toml", "deny.toml"]),
            ),
            ("workspace/crates", dir_entry(&["core"], &[])),
            (
                "workspace/crates/core",
                dir_entry(&[], &["Cargo.toml", file_name]),
            ),
        ],
        vec![
            (
                "workspace/Cargo.toml",
                "[workspace]\nmembers=[\"crates/*\"]".to_owned(),
            ),
            (
                "workspace/crates/core/Cargo.toml",
                "[package]\nname=\"core\"".to_owned(),
            ),
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

pub fn config_facts(deny: &str) -> DenyConfigFacts {
    let (parsed, parse_error) = match toml::from_str::<toml::Value>(deny) {
        Ok(parsed) => (Some(parsed), None),
        Err(err) => (None, Some(err.to_string())),
    };
    DenyConfigFacts {
        policy_root_rel: String::new(),
        rel_path: "deny.toml".to_owned(),
        file_kind: "deny.toml".to_owned(),
        parsed,
        parse_error,
        profile_name: Some("service".to_owned()),
    }
}

pub fn collected_facts(tree: &ProjectTree) -> DenyFacts {
    collect(tree)
}

pub fn forbidden_input<'a>(facts: &'a DenyFacts, rel_path: &str) -> ForbiddenDenyConfigInput<'a> {
    let forbidden = facts
        .forbidden_configs
        .iter()
        .find(|config| config.rel_path == rel_path)
        .expect("expected forbidden deny config facts");
    ForbiddenDenyConfigInput::new(forbidden)
}

pub fn same_root_conflict_input<'a>(
    facts: &'a DenyFacts,
    policy_root_rel: &str,
) -> SameRootConflictInput<'a> {
    let conflict = facts
        .same_root_conflicts
        .iter()
        .find(|conflict| conflict.policy_root_rel == policy_root_rel)
        .expect("expected same-root deny conflict facts");
    SameRootConflictInput::new(conflict)
}

pub fn covered_input<'a>(facts: &'a DenyFacts, rel_dir: &str) -> CoveredRustUnitInput<'a> {
    let covered = facts
        .covered_units
        .iter()
        .find(|covered| covered.rel_dir == rel_dir)
        .expect("expected covered rust unit facts");
    CoveredRustUnitInput::new(covered)
}

pub fn uncovered_input<'a>(facts: &'a DenyFacts, rel_dir: &str) -> UncoveredRustUnitInput<'a> {
    let uncovered = facts
        .uncovered_units
        .iter()
        .find(|unit| unit.rel_dir == rel_dir)
        .expect("expected uncovered rust unit facts");
    UncoveredRustUnitInput::new(uncovered)
}
