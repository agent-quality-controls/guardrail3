use std::collections::BTreeMap;
use std::path::PathBuf;

use crate::domain::project_tree::{DirEntry, ProjectTree};

use super::check;

#[test]
fn root_rustfmt_config_is_checked_and_extra_configs_are_fanned_out() {
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
                r#"
                    [workspace.package]
                    edition = "2024"
                "#
                .to_owned(),
            ),
            (
                "rustfmt.toml".to_owned(),
                r#"
                    edition = "2024"
                    max_width = 100
                    tab_spaces = 4
                    use_field_init_shorthand = true
                    use_try_shorthand = true
                    reorder_imports = true
                    reorder_modules = true
                "#
                .to_owned(),
            ),
            (
                "crates/core/.rustfmt.toml".to_owned(),
                "max_width = 120".to_owned(),
            ),
        ]),
    };

    let results = check(&tree);
    assert!(results.iter().any(|r| r.id == "RS-FMT-01"));
    assert!(
        results
            .iter()
            .any(|r| r.id == "RS-FMT-05" && r.file.as_deref() == Some("crates/core/.rustfmt.toml"))
    );
}

#[test]
fn dual_root_config_conflict_is_reported() {
    let tree = ProjectTree {
        root: PathBuf::from("/tmp/project"),
        structure: BTreeMap::from([(
            "".to_owned(),
            DirEntry {
                dirs: vec![],
                files: vec![
                    "Cargo.toml".to_owned(),
                    "rustfmt.toml".to_owned(),
                    ".rustfmt.toml".to_owned(),
                ],
            },
        )]),
        content: BTreeMap::from([
            ("Cargo.toml".to_owned(), "[package]\nedition = \"2024\"".to_owned()),
            (
                "rustfmt.toml".to_owned(),
                "edition = \"2024\"\nmax_width = 100\ntab_spaces = 4\nuse_field_init_shorthand = true\nuse_try_shorthand = true\nreorder_imports = true\nreorder_modules = true".to_owned(),
            ),
            (".rustfmt.toml".to_owned(), "max_width = 120".to_owned()),
        ]),
    };

    let results = check(&tree);
    assert!(results.iter().any(|r| r.id == "RS-FMT-08"));
}

#[test]
fn missing_root_config_reports_error() {
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
            "[package]\nedition = \"2024\"".to_owned(),
        )]),
    };

    let results = check(&tree);
    assert!(results.iter().any(|r| r.id == "RS-FMT-01" && !r.inventory));
}

#[test]
fn nightly_keys_ignore_and_edition_mismatch_are_reported() {
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
                r#"
                    edition = "2021"
                    max_width = 100
                    tab_spaces = 4
                    use_field_init_shorthand = true
                    use_try_shorthand = true
                    reorder_imports = true
                    reorder_modules = true
                    group_imports = "StdExternalCrate"
                    ignore = ["generated/**"]
                "#
                .to_owned(),
            ),
        ]),
    };

    let results = check(&tree);
    assert!(results.iter().any(|r| r.id == "RS-FMT-04"));
    assert!(results.iter().any(|r| r.id == "RS-FMT-06"));
    assert!(results.iter().any(|r| r.id == "RS-FMT-07"));
    assert!(
        !results
            .iter()
            .any(|r| r.id == "RS-FMT-03" && r.title.contains("ignore"))
    );
}
