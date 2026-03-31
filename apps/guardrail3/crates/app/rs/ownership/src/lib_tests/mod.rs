use std::collections::BTreeMap;
use std::path::PathBuf;

use guardrail3_app_rs_placement::collect as collect_placement;
use guardrail3_domain_project_tree::{DirEntry, ProjectTree};
use guardrail3_validation_model::RustValidateFamily;

use crate::{RustFamilyFileAttachment, RustFamilyFileKind, collect};

#[test]
fn attaches_toolchain_file_to_exact_root() {
    let tree = project_tree(
        vec![(
            "apps/backend",
            dir_entry(&[], &["Cargo.toml", "rust-toolchain.toml"]),
        )],
        vec![(
            "apps/backend/Cargo.toml",
            "[workspace]\nmembers = []\nresolver = \"2\"\n",
        )],
    );

    let facts = collect(&tree, &collect_placement(&tree));
    let file = find_file(
        &facts,
        RustValidateFamily::Toolchain,
        "apps/backend/rust-toolchain.toml",
    );

    assert_eq!(file.kind(), RustFamilyFileKind::RustToolchainToml);
    assert_eq!(
        file.attachment(),
        &RustFamilyFileAttachment::ExactRoot {
            root_rel: "apps/backend".to_owned(),
        }
    );
}

#[test]
fn attaches_validation_root_policy_file_as_ancestor_of_descendant_roots() {
    let tree = project_tree(
        vec![
            ("", dir_entry(&["apps"], &["clippy.toml"])),
            ("apps", dir_entry(&["backend"], &[])),
            ("apps/backend", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![
            ("clippy.toml", "msrv = \"1.85\"\n"),
            (
                "apps/backend/Cargo.toml",
                "[workspace]\nmembers = []\nresolver = \"2\"\n",
            ),
        ],
    );

    let facts = collect(&tree, &collect_placement(&tree));
    let file = find_file(&facts, RustValidateFamily::Clippy, "clippy.toml");

    assert_eq!(file.kind(), RustFamilyFileKind::ClippyToml);
    assert_eq!(
        file.attachment(),
        &RustFamilyFileAttachment::AncestorOfRoots {
            root_rels: vec!["apps/backend".to_owned()],
            owner_rel: String::new(),
        }
    );
}

#[test]
fn normalizes_cargo_config_owner_to_parent_root() {
    let tree = project_tree(
        vec![
            ("apps/backend", dir_entry(&[".cargo"], &["Cargo.toml"])),
            ("apps/backend/.cargo", dir_entry(&[], &["config.toml"])),
        ],
        vec![(
            "apps/backend/Cargo.toml",
            "[workspace]\nmembers = []\nresolver = \"2\"\n",
        )],
    );

    let facts = collect(&tree, &collect_placement(&tree));
    let file = find_file(
        &facts,
        RustValidateFamily::Clippy,
        "apps/backend/.cargo/config.toml",
    );

    assert_eq!(file.kind(), RustFamilyFileKind::CargoConfigToml);
    assert_eq!(
        file.attachment(),
        &RustFamilyFileAttachment::ExactRoot {
            root_rel: "apps/backend".to_owned(),
        }
    );
}

#[test]
fn attaches_nested_member_config_under_nearest_root() {
    let tree = project_tree(
        vec![
            ("workspace", dir_entry(&["crates"], &["Cargo.toml"])),
            ("workspace/crates", dir_entry(&["core"], &[])),
            ("workspace/crates/core", dir_entry(&[".cargo"], &[])),
            (
                "workspace/crates/core/.cargo",
                dir_entry(&[], &["deny.toml"]),
            ),
        ],
        vec![(
            "workspace/Cargo.toml",
            "[workspace]\nmembers = [\"crates/core\"]\nresolver = \"2\"\n",
        )],
    );

    let facts = collect(&tree, &collect_placement(&tree));
    let file = find_file(
        &facts,
        RustValidateFamily::Deny,
        "workspace/crates/core/.cargo/deny.toml",
    );

    assert_eq!(file.kind(), RustFamilyFileKind::CargoDenyToml);
    assert_eq!(
        file.attachment(),
        &RustFamilyFileAttachment::NestedUnderRoot {
            root_rel: "workspace".to_owned(),
            owner_rel: "workspace/crates/core".to_owned(),
        }
    );
}

#[test]
fn keeps_stray_toolchain_outside_all_roots_visible() {
    let tree = project_tree(
        vec![
            ("apps", dir_entry(&["backend"], &[])),
            ("apps/backend", dir_entry(&[], &["Cargo.toml"])),
            ("tools", dir_entry(&["helper"], &[])),
            ("tools/helper", dir_entry(&[], &["rust-toolchain"])),
        ],
        vec![(
            "apps/backend/Cargo.toml",
            "[workspace]\nmembers = []\nresolver = \"2\"\n",
        )],
    );

    let facts = collect(&tree, &collect_placement(&tree));
    let file = find_file(
        &facts,
        RustValidateFamily::Toolchain,
        "tools/helper/rust-toolchain",
    );

    assert_eq!(file.kind(), RustFamilyFileKind::RustToolchainLegacy);
    assert_eq!(
        file.attachment(),
        &RustFamilyFileAttachment::OutsideRoots {
            owner_rel: "tools/helper".to_owned(),
        }
    );
}

fn find_file<'a>(
    facts: &'a crate::RustOwnedSurfaceFacts,
    family: RustValidateFamily,
    rel_path: &str,
) -> &'a crate::RustFamilyFileFact {
    facts
        .family_files()
        .iter()
        .find(|file| file.family() == family && file.rel_path() == rel_path)
        .unwrap_or_else(|| panic!("missing {family:?} file fact for {rel_path}"))
}

fn project_tree(dirs: Vec<(&str, DirEntry)>, content: Vec<(&str, &str)>) -> ProjectTree {
    let mut structure = BTreeMap::from([("".to_owned(), dir_entry(&[], &[]))]);
    for (rel, entry) in dirs {
        let _ = structure.insert(rel.to_owned(), entry);
    }

    ProjectTree::new(
        PathBuf::from("/tmp/ownership-tests"),
        structure,
        content
            .into_iter()
            .map(|(rel, body)| (rel.to_owned(), body.to_owned()))
            .collect(),
    )
}

fn dir_entry(dirs: &[&str], files: &[&str]) -> DirEntry {
    DirEntry::new(
        dirs.iter().map(|value| (*value).to_owned()).collect(),
        files.iter().map(|value| (*value).to_owned()).collect(),
        Vec::new(),
        Vec::new(),
    )
}
