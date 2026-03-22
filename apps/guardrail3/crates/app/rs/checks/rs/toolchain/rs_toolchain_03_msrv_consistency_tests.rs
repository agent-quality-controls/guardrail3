use std::collections::BTreeMap;
use std::path::PathBuf;

use crate::domain::project_tree::{DirEntry, ProjectTree};

use super::super::check;

#[test]
fn warns_when_pinned_toolchain_is_older_than_msrv() {
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
                "[toolchain]\nchannel = \"1.84.0\"\ncomponents = [\"clippy\", \"rustfmt\"]"
                    .to_owned(),
            ),
        ]),
    };

    let results = check(&tree);
    assert!(
        results
            .iter()
            .any(|r| r.id == "RS-TOOLCHAIN-03" && !r.inventory)
    );
}
