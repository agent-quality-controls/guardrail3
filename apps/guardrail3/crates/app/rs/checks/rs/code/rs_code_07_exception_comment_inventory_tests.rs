use crate::domain::report::Severity;
use crate::domain::project_tree::{DirEntry, ProjectTree};

use std::collections::BTreeMap;
use std::path::PathBuf;

use super::super::facts::collect;
use super::super::inputs::ExceptionCommentInput;
use super::check;

#[test]
fn inventories_exception_comment() {
    let input = ExceptionCommentInput {
        rel_path: "Cargo.toml",
        line: 4,
        line_text: "# EXCEPTION: temporary override",
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-CODE-07");
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
}

#[test]
fn collects_exception_comments_from_nested_config_files() {
    let tree = ProjectTree {
        root: PathBuf::from("/tmp/guardrail3-rs-code-07"),
        structure: BTreeMap::from([
            (
                String::new(),
                DirEntry {
                    dirs: vec!["packages".to_owned()],
                    files: vec!["Cargo.toml".to_owned()],
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
                    dirs: Vec::new(),
                    files: vec!["clippy.toml".to_owned()],
                },
            ),
        ]),
        content: BTreeMap::from([
            ("Cargo.toml".to_owned(), "[workspace]\nmembers = [\"packages/*\"]\n".to_owned()),
            (
                "packages/demo/clippy.toml".to_owned(),
                "disallowed-methods = []\n# EXCEPTION: package-local audit note\n".to_owned(),
            ),
        ]),
    };

    let facts = collect(&tree);

    assert!(facts.exception_comments.iter().any(|comment| {
        comment.rel_path == "packages/demo/clippy.toml"
            && comment.line_text == "# EXCEPTION: package-local audit note"
    }));
}
