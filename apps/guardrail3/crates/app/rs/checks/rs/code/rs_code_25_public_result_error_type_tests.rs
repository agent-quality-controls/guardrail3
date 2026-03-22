use crate::domain::report::Severity;
use crate::domain::project_tree::{DirEntry, ProjectTree};

use super::super::facts::collect;
use super::super::inputs::RustCodeFileInput;
use super::super::parse::parse_rust_file;
use super::check;
use std::collections::BTreeMap;
use std::path::PathBuf;

#[test]
fn warns_on_public_result_string_in_library_profile() {
    let content = "pub fn parse() -> Result<(), String> { Ok(()) }";
    let ast = parse_rust_file(content).expect("valid rust");
    let input = RustCodeFileInput {
        rel_path: "src/lib.rs",
        content,
        ast: &ast,
        is_test: false,
        profile_name: Some("library"),
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-CODE-25");
    assert_eq!(results[0].severity, Severity::Warn);
}

#[test]
fn warns_for_workspace_member_package_under_rust_packages_profile() {
    let root = temp_root("rs-code-25-workspace-member-package");
    let source_rel = "packages/demo/src/lib.rs";
    let source_abs = root.join(source_rel);
    std::fs::create_dir_all(source_abs.parent().expect("parent")).expect("create source dir");
    std::fs::write(&source_abs, "pub fn parse() -> Result<(), String> { Ok(()) }")
        .expect("write source");

    let tree = ProjectTree {
        root: root.clone(),
        structure: BTreeMap::from([
            (
                String::new(),
                DirEntry {
                    dirs: vec!["packages".to_owned()],
                    files: vec!["Cargo.toml".to_owned(), "guardrail3.toml".to_owned()],
                },
            ),
            (
                "packages".to_owned(),
                DirEntry {
                    dirs: vec!["demo".to_owned()],
                    files: Vec::new(),
                },
            ),
            (
                "packages/demo".to_owned(),
                DirEntry {
                    dirs: vec!["src".to_owned()],
                    files: vec!["Cargo.toml".to_owned()],
                },
            ),
            (
                "packages/demo/src".to_owned(),
                DirEntry {
                    dirs: Vec::new(),
                    files: vec!["lib.rs".to_owned()],
                },
            ),
        ]),
        content: BTreeMap::from([
            (
                "Cargo.toml".to_owned(),
                "[workspace]\nmembers = [\"packages/*\"]\n".to_owned(),
            ),
            (
                "packages/demo/Cargo.toml".to_owned(),
                "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n".to_owned(),
            ),
            (
                "guardrail3.toml".to_owned(),
                "[rust.packages]\ntype = \"library\"\n".to_owned(),
            ),
        ]),
    };

    let facts = collect(&tree);
    let file = facts
        .files
        .iter()
        .find(|file| file.rel_path == source_rel)
        .expect("source file facts");

    assert_eq!(file.profile_name.as_deref(), Some("library"));

    std::fs::remove_dir_all(&root).expect("remove temp tree");
}

fn temp_root(slug: &str) -> PathBuf {
    let unique = format!(
        "{}-{}-{}",
        slug,
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock")
            .as_nanos()
    );
    std::env::temp_dir().join(unique)
}
