use std::collections::BTreeMap;
use std::path::PathBuf;

use crate::domain::project_tree::{DirEntry, ProjectTree};

use super::check;

#[test]
fn stable_toolchain_with_components_passes() {
    let tree = ProjectTree {
        root: PathBuf::from("/tmp/project"),
        structure: BTreeMap::from([(
            "".to_owned(),
            DirEntry {
                dirs: vec![],
                files: vec!["Cargo.toml".to_owned(), "rust-toolchain.toml".to_owned()],
            },
        )]),
        content: BTreeMap::from([
            (
                "Cargo.toml".to_owned(),
                "[package]\nrust-version = \"1.85\"".to_owned(),
            ),
            (
                "rust-toolchain.toml".to_owned(),
                r#"
                    [toolchain]
                    channel = "stable"
                    components = ["clippy", "rustfmt"]
                "#
                .to_owned(),
            ),
        ]),
    };

    let results = check(&tree);
    assert!(results.iter().any(|r| r.id == "RS-TOOLCHAIN-01" && r.inventory));
    assert!(results.iter().any(|r| r.id == "RS-TOOLCHAIN-02" && r.inventory));
}

#[test]
fn pinned_toolchain_older_than_msrv_is_warned() {
    let tree = ProjectTree {
        root: PathBuf::from("/tmp/project"),
        structure: BTreeMap::from([(
            "".to_owned(),
            DirEntry {
                dirs: vec![],
                files: vec!["Cargo.toml".to_owned(), "rust-toolchain.toml".to_owned()],
            },
        )]),
        content: BTreeMap::from([
            (
                "Cargo.toml".to_owned(),
                "[package]\nrust-version = \"1.85.0\"".to_owned(),
            ),
            (
                "rust-toolchain.toml".to_owned(),
                r#"
                    [toolchain]
                    channel = "1.84.0"
                    components = ["clippy", "rustfmt"]
                "#
                .to_owned(),
            ),
        ]),
    };

    let results = check(&tree);
    assert!(results.iter().any(|r| r.id == "RS-TOOLCHAIN-03" && !r.inventory));
}

#[test]
fn legacy_toolchain_file_is_warned() {
    let tree = ProjectTree {
        root: PathBuf::from("/tmp/project"),
        structure: BTreeMap::from([(
            "".to_owned(),
            DirEntry {
                dirs: vec![],
                files: vec!["rust-toolchain".to_owned()],
            },
        )]),
        content: BTreeMap::new(),
    };

    let results = check(&tree);
    assert!(results.iter().any(|r| r.id == "RS-TOOLCHAIN-01" && !r.inventory));
    assert!(results.iter().any(|r| r.id == "RS-TOOLCHAIN-04"));
}

#[test]
fn duplicate_toolchain_files_are_warned() {
    let tree = ProjectTree {
        root: PathBuf::from("/tmp/project"),
        structure: BTreeMap::from([(
            "".to_owned(),
            DirEntry {
                dirs: vec![],
                files: vec!["rust-toolchain".to_owned(), "rust-toolchain.toml".to_owned()],
            },
        )]),
        content: BTreeMap::from([(
            "rust-toolchain.toml".to_owned(),
            r#"
                [toolchain]
                channel = "stable"
                components = ["clippy", "rustfmt"]
            "#
            .to_owned(),
        )]),
    };

    let results = check(&tree);
    assert!(results.iter().any(|r| r.id == "RS-TOOLCHAIN-04"));
}
