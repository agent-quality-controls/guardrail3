use std::collections::BTreeMap;
use std::path::PathBuf;

use crate::domain::project_tree::{DirEntry, ProjectTree};

use super::super::check;

#[test]
fn reports_missing_required_root_settings() {
    let tree = ProjectTree {
        root: PathBuf::from("/tmp/project"),
        structure: BTreeMap::from([(
            "".to_owned(),
            DirEntry {
                dirs: vec![],
                files: vec!["Cargo.toml".to_owned(), "rustfmt.toml".to_owned()],
            },
        )]),
        content: BTreeMap::from([
            (
                "Cargo.toml".to_owned(),
                "[package]\nedition = \"2024\"".to_owned(),
            ),
            ("rustfmt.toml".to_owned(), "edition = \"2024\"".to_owned()),
        ]),
    };

    let results = check(&tree);
    assert!(results.iter().any(|r| r.id == "RS-FMT-02" && !r.inventory));
}
