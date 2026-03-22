use std::collections::BTreeMap;
use std::path::PathBuf;

use crate::domain::project_tree::{DirEntry, ProjectTree};

use super::super::check;

#[test]
fn reports_nightly_only_keys_on_stable_toolchain() {
    let tree = ProjectTree {
        root: PathBuf::from("/tmp/project"),
        structure: BTreeMap::from([(
            "".to_owned(),
            DirEntry {
                dirs: vec![],
                files: vec![
                    "Cargo.toml".to_owned(),
                    "rustfmt.toml".to_owned(),
                    "rust-toolchain.toml".to_owned(),
                ],
            },
        )]),
        content: BTreeMap::from([
            (
                "Cargo.toml".to_owned(),
                "[workspace.package]\nedition = \"2024\"".to_owned(),
            ),
            (
                "rust-toolchain.toml".to_owned(),
                "[toolchain]\nchannel = \"stable\"".to_owned(),
            ),
            (
                "rustfmt.toml".to_owned(),
                "edition = \"2024\"\nmax_width = 100\ntab_spaces = 4\nuse_field_init_shorthand = true\nuse_try_shorthand = true\nreorder_imports = true\nreorder_modules = true\ngroup_imports = \"StdExternalCrate\"".to_owned(),
            ),
        ]),
    };

    let results = check(&tree);
    assert!(results.iter().any(|r| r.id == "RS-FMT-04"));
}
