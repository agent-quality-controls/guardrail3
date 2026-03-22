use std::collections::BTreeMap;
use std::path::PathBuf;

use crate::domain::project_tree::{DirEntry, ProjectTree};

use super::super::check;

#[test]
fn reports_extra_per_crate_rustfmt_configs() {
    let tree = ProjectTree {
        root: PathBuf::from("/tmp/project"),
        structure: BTreeMap::from([
            (
                "".to_owned(),
                DirEntry {
                    dirs: vec!["crates".to_owned()],
                    files: vec!["Cargo.toml".to_owned(), "rustfmt.toml".to_owned()],
                },
            ),
            (
                "crates".to_owned(),
                DirEntry {
                    dirs: vec!["core".to_owned()],
                    files: vec![],
                },
            ),
            (
                "crates/core".to_owned(),
                DirEntry {
                    dirs: vec![],
                    files: vec![".rustfmt.toml".to_owned()],
                },
            ),
        ]),
        content: BTreeMap::from([
            (
                "Cargo.toml".to_owned(),
                "[workspace.package]\nedition = \"2024\"".to_owned(),
            ),
            (
                "rustfmt.toml".to_owned(),
                "edition = \"2024\"\nmax_width = 100\ntab_spaces = 4\nuse_field_init_shorthand = true\nuse_try_shorthand = true\nreorder_imports = true\nreorder_modules = true".to_owned(),
            ),
            (
                "crates/core/.rustfmt.toml".to_owned(),
                "max_width = 120".to_owned(),
            ),
        ]),
    };

    let results = check(&tree);
    assert!(
        results
            .iter()
            .any(|r| r.id == "RS-FMT-05" && r.file.as_deref() == Some("crates/core/.rustfmt.toml"))
    );
}
