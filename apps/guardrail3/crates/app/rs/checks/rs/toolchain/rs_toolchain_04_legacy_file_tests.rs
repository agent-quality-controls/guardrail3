use std::collections::BTreeMap;
use std::path::PathBuf;

use crate::domain::project_tree::{DirEntry, ProjectTree};
use crate::domain::report::Severity;

use super::super::check;

#[test]
fn warns_when_only_legacy_toolchain_file_exists() {
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
    assert!(results.iter().any(|result| {
        result.id == "RS-TOOLCHAIN-04"
            && result.severity == Severity::Warn
            && result.title == "legacy rust-toolchain file present"
            && result.file.as_deref() == Some("rust-toolchain")
    }));
}

#[test]
fn warns_when_both_legacy_and_modern_toolchain_files_exist() {
    let tree = ProjectTree {
        root: PathBuf::from("/tmp/project"),
        structure: BTreeMap::from([(
            "".to_owned(),
            DirEntry {
                dirs: vec![],
                files: vec![
                    "rust-toolchain".to_owned(),
                    "rust-toolchain.toml".to_owned(),
                ],
            },
        )]),
        content: BTreeMap::from([(
            "rust-toolchain.toml".to_owned(),
            "[toolchain]\nchannel = \"stable\"\ncomponents = [\"clippy\", \"rustfmt\"]"
                .to_owned(),
        )]),
    };

    let results = check(&tree);
    assert!(results.iter().any(|result| {
        result.id == "RS-TOOLCHAIN-04"
            && result.severity == Severity::Warn
            && result.title == "both rust-toolchain files present"
            && result.file.as_deref() == Some("rust-toolchain")
    }));
}
