use std::collections::BTreeMap;
use std::path::PathBuf;

use crate::domain::project_tree::{DirEntry, ProjectTree};

use super::super::check;

#[test]
fn inventories_when_toolchain_toml_exists() {
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
                "[toolchain]\nchannel = \"stable\"\ncomponents = [\"clippy\", \"rustfmt\"]"
                    .to_owned(),
            ),
        ]),
    };

    let results = check(&tree);
    assert!(
        results
            .iter()
            .any(|r| r.id == "RS-TOOLCHAIN-01" && r.inventory)
    );
}

#[test]
fn errors_when_no_supported_toolchain_file_exists() {
    let tree = ProjectTree {
        root: PathBuf::from("/tmp/project"),
        structure: BTreeMap::from([(
            "".to_owned(),
            DirEntry {
                dirs: vec![],
                files: vec!["Cargo.toml".to_owned()],
            },
        )]),
        content: BTreeMap::from([(
            "Cargo.toml".to_owned(),
            "[package]\nrust-version = \"1.85\"".to_owned(),
        )]),
    };

    let results = check(&tree);
    assert!(
        results
            .iter()
            .any(|r| r.id == "RS-TOOLCHAIN-01" && !r.inventory)
    );
}
