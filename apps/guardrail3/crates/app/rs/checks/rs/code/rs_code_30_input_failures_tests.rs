use std::collections::BTreeMap;
use std::path::PathBuf;

use crate::domain::project_tree::{DirEntry, ProjectTree};
use crate::domain::report::Severity;

use super::super::inputs::CodeInputFailureInput;
use super::check;

#[test]
fn errors_on_direct_input_failure() {
    let input = CodeInputFailureInput {
        rel_path: "src/lib.rs",
        message: "Failed to parse Rust source file: unexpected token",
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-CODE-30");
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(
        results[0].message,
        "Failed to parse Rust source file: unexpected token"
    );
}

#[test]
fn family_surfaces_source_parse_failures_instead_of_skipping() {
    let root = temp_root("rs-code-30-source-parse-failure");
    let source_rel = "src/lib.rs";
    let source_abs = root.join(source_rel);
    std::fs::create_dir_all(source_abs.parent().expect("parent")).expect("create source dir");
    std::fs::write(&source_abs, "fn broken( {").expect("write source");

    let tree = ProjectTree {
        root: root.clone(),
        structure: BTreeMap::from([
            (
                String::new(),
                DirEntry {
                    dirs: vec!["src".to_owned()],
                    files: vec!["Cargo.toml".to_owned()],
                },
            ),
            (
                "src".to_owned(),
                DirEntry {
                    dirs: Vec::new(),
                    files: vec!["lib.rs".to_owned()],
                },
            ),
        ]),
        content: BTreeMap::from([(
            "Cargo.toml".to_owned(),
            "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n".to_owned(),
        )]),
    };

    let results = crate::app::rs::checks::rs::code::check(&tree);

    assert!(results.iter().any(|result| {
        result.id == "RS-CODE-30"
            && result.file.as_deref() == Some(source_rel)
            && result.severity == Severity::Error
            && result.message.contains("Failed to parse Rust source file")
    }));

    std::fs::remove_dir_all(&root).expect("remove temp tree");
}

#[test]
fn family_surfaces_guardrail_policy_parse_failures() {
    let root = temp_root("rs-code-30-guardrail-parse-failure");
    let source_rel = "src/lib.rs";
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
                    dirs: vec!["src".to_owned()],
                    files: vec!["Cargo.toml".to_owned(), "guardrail3.toml".to_owned()],
                },
            ),
            (
                "src".to_owned(),
                DirEntry {
                    dirs: Vec::new(),
                    files: vec!["lib.rs".to_owned()],
                },
            ),
        ]),
        content: BTreeMap::from([
            (
                "Cargo.toml".to_owned(),
                "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n".to_owned(),
            ),
            (
                "guardrail3.toml".to_owned(),
                "[rust.packages\n type = \"library\"\n".to_owned(),
            ),
        ]),
    };

    let results = crate::app::rs::checks::rs::code::check(&tree);

    assert!(results.iter().any(|result| {
        result.id == "RS-CODE-30"
            && result.file.as_deref() == Some("guardrail3.toml")
            && result.severity == Severity::Error
            && result.message.contains("Failed to parse guardrail3.toml")
    }));

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
