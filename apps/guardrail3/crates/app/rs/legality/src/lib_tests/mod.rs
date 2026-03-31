use std::collections::BTreeMap;
use std::path::PathBuf;

use guardrail3_app_rs_ownership::RustFamilyFileKind;
use guardrail3_app_rs_structure::collect as collect_structure;
use guardrail3_domain_project_tree::{DirEntry, ProjectTree};
use guardrail3_validation_model::RustValidateFamily;

use crate::{
    RustIllegalFamilyFileReason, RustTopologyIssueKind, collect as collect_legality,
};

#[test]
fn legal_workspace_root_and_member_cargo_are_kept_legal() {
    let tree = workspace_tree();
    let structure = collect_structure(&tree);
    let legality = collect_legality(&tree, &structure);

    let legal_roots = legality
        .legal_workspace_roots()
        .iter()
        .map(|root| root.rel_dir().to_owned())
        .collect::<Vec<_>>();
    assert_eq!(legal_roots, vec!["apps/api".to_owned()]);

    let clippy_cargo_files = legality
        .legal_family_files()
        .iter()
        .filter(|file| {
            file.family() == RustValidateFamily::Clippy
                && file.kind() == RustFamilyFileKind::CargoToml
        })
        .map(|file| file.rel_path().to_owned())
        .collect::<Vec<_>>();
    assert_eq!(
        clippy_cargo_files,
        vec![
            "apps/api/Cargo.toml".to_owned(),
            "apps/api/crates/member/Cargo.toml".to_owned(),
        ]
    );
}

#[test]
fn nested_workspace_and_nested_clippy_are_illegal() {
    let tree = nested_workspace_tree();
    let structure = collect_structure(&tree);
    let legality = collect_legality(&tree, &structure);

    assert!(legality.topology_issues().iter().any(|issue| {
        issue.rel_dir() == "apps/api/crates/demo"
            && matches!(
                issue.kind(),
                RustTopologyIssueKind::NestedWorkspace { parent_workspace_rel }
                if parent_workspace_rel == "apps/api"
            )
    }));
    assert!(legality.illegal_family_files().iter().any(|file| {
        file.family() == RustValidateFamily::Clippy
            && file.rel_path() == "apps/api/crates/demo/clippy.toml"
            && matches!(
                file.reason(),
                RustIllegalFamilyFileReason::AttachedToIllegalRoot { root_rel }
                if root_rel == "apps/api/crates/demo"
            )
    }));
}

#[test]
fn top_level_package_toolchain_is_illegal_for_local_family_routing() {
    let tree = ProjectTree::new(
        PathBuf::from("/tmp/project"),
        BTreeMap::from([
            (
                "".to_owned(),
                DirEntry::new(vec!["tools".to_owned()], Vec::new(), Vec::new(), Vec::new()),
            ),
            (
                "tools".to_owned(),
                DirEntry::new(vec!["helper".to_owned()], Vec::new(), Vec::new(), Vec::new()),
            ),
            (
                "tools/helper".to_owned(),
                DirEntry::new(
                    Vec::new(),
                    vec!["Cargo.toml".to_owned(), "rust-toolchain.toml".to_owned()],
                    Vec::new(),
                    Vec::new(),
                ),
            ),
        ]),
        BTreeMap::from([
            (
                "tools/helper/Cargo.toml".to_owned(),
                "[package]\nname = \"helper\"\nversion = \"0.1.0\"\n".to_owned(),
            ),
            (
                "tools/helper/rust-toolchain.toml".to_owned(),
                "[toolchain]\nchannel = \"stable\"\n".to_owned(),
            ),
        ]),
    );
    let structure = collect_structure(&tree);
    let legality = collect_legality(&tree, &structure);

    assert!(legality.topology_issues().iter().any(|issue| {
        issue.rel_dir() == "tools/helper"
            && matches!(issue.kind(), RustTopologyIssueKind::LooseTopLevelPackage)
    }));
    assert!(legality.illegal_family_files().iter().any(|file| {
        file.family() == RustValidateFamily::Toolchain
            && file.rel_path() == "tools/helper/rust-toolchain.toml"
            && matches!(
                file.reason(),
                RustIllegalFamilyFileReason::AttachedToIllegalRoot { root_rel }
                if root_rel == "tools/helper"
            )
    }));
}

fn workspace_tree() -> ProjectTree {
    ProjectTree::new(
        PathBuf::from("/tmp/project"),
        BTreeMap::from([
            (
                "".to_owned(),
                DirEntry::new(vec!["apps".to_owned()], Vec::new(), Vec::new(), Vec::new()),
            ),
            (
                "apps".to_owned(),
                DirEntry::new(vec!["api".to_owned()], Vec::new(), Vec::new(), Vec::new()),
            ),
            (
                "apps/api".to_owned(),
                DirEntry::new(
                    vec!["crates".to_owned()],
                    vec!["Cargo.toml".to_owned(), "clippy.toml".to_owned()],
                    Vec::new(),
                    Vec::new(),
                ),
            ),
            (
                "apps/api/crates".to_owned(),
                DirEntry::new(vec!["member".to_owned()], Vec::new(), Vec::new(), Vec::new()),
            ),
            (
                "apps/api/crates/member".to_owned(),
                DirEntry::new(
                    Vec::new(),
                    vec!["Cargo.toml".to_owned()],
                    Vec::new(),
                    Vec::new(),
                ),
            ),
        ]),
        BTreeMap::from([
            (
                "apps/api/Cargo.toml".to_owned(),
                "[workspace]\nmembers = [\"crates/member\"]\nresolver = \"2\"\n".to_owned(),
            ),
            (
                "apps/api/crates/member/Cargo.toml".to_owned(),
                "[package]\nname = \"member\"\nversion = \"0.1.0\"\n".to_owned(),
            ),
            ("apps/api/clippy.toml".to_owned(), "msrv = \"1.85\"\n".to_owned()),
        ]),
    )
}

fn nested_workspace_tree() -> ProjectTree {
    ProjectTree::new(
        PathBuf::from("/tmp/project"),
        BTreeMap::from([
            (
                "".to_owned(),
                DirEntry::new(vec!["apps".to_owned()], Vec::new(), Vec::new(), Vec::new()),
            ),
            (
                "apps".to_owned(),
                DirEntry::new(vec!["api".to_owned()], Vec::new(), Vec::new(), Vec::new()),
            ),
            (
                "apps/api".to_owned(),
                DirEntry::new(
                    vec!["crates".to_owned()],
                    vec!["Cargo.toml".to_owned()],
                    Vec::new(),
                    Vec::new(),
                ),
            ),
            (
                "apps/api/crates".to_owned(),
                DirEntry::new(vec!["demo".to_owned()], Vec::new(), Vec::new(), Vec::new()),
            ),
            (
                "apps/api/crates/demo".to_owned(),
                DirEntry::new(
                    Vec::new(),
                    vec!["Cargo.toml".to_owned(), "clippy.toml".to_owned()],
                    Vec::new(),
                    Vec::new(),
                ),
            ),
        ]),
        BTreeMap::from([
            (
                "apps/api/Cargo.toml".to_owned(),
                "[workspace]\nmembers = []\nresolver = \"2\"\n".to_owned(),
            ),
            (
                "apps/api/crates/demo/Cargo.toml".to_owned(),
                "[workspace]\nmembers = []\nresolver = \"2\"\n".to_owned(),
            ),
            ("apps/api/crates/demo/clippy.toml".to_owned(), "msrv = \"1.85\"\n".to_owned()),
        ]),
    )
}
