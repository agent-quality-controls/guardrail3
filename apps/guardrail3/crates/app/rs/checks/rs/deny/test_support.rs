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
    crate::domain::modules::deny::build_deny_toml("service", "", "", "")
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
