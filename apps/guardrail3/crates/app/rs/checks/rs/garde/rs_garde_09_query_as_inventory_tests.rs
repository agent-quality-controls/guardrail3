use crate::domain::report::Severity;

use super::super::inputs::QueryAsMacroInput;
use super::super::test_support::{dir_entry, has_result, project_tree, query_as_macro, temp_root};
use super::check;

#[test]
fn inventories_query_as_usage() {
    let mut results = Vec::new();
    check(&QueryAsMacroInput::new(&query_as_macro()), &mut results);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-GARDE-09");
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
}

#[test]
fn family_inventories_query_as_macro_usage() {
    let root = temp_root("rs-garde-09-family");
    let source_rel = "src/db.rs";
    let source_abs = root.join(source_rel);
    std::fs::create_dir_all(source_abs.parent().expect("parent")).expect("mkdir");
    std::fs::write(
        &source_abs,
        r#"
fn load() {
    let _row = sqlx::query_as!(User, "select 1");
}
"#,
    )
    .expect("write");

    let tree = project_tree(
        vec![
            (
                "",
                dir_entry(&["src"], &["Cargo.toml", "clippy.toml", "guardrail3.toml"]),
            ),
            ("src", dir_entry(&[], &["db.rs"])),
        ],
        vec![
            (
                "Cargo.toml",
                r#"[workspace]
members = []
[workspace.dependencies]
garde = { version = "0.22", features = ["derive"] }
"#,
            ),
            ("clippy.toml", "disallowed-methods = []\ndisallowed-types = []\n"),
            ("guardrail3.toml", "[profile]\nname = \"service\"\n"),
        ],
        root.clone(),
    );

    let results = crate::app::rs::checks::rs::garde::check(&tree);
    assert!(has_result(&results, "RS-GARDE-09", |result| {
        result.severity == Severity::Info
            && result.inventory
            && result.file.as_deref() == Some(source_rel)
    }));

    std::fs::remove_dir_all(root).expect("cleanup");
}

#[test]
fn family_inventories_aliased_query_as_macro_usage() {
    let root = temp_root("rs-garde-09-aliased-family");
    let source_rel = "src/db.rs";
    let source_abs = root.join(source_rel);
    std::fs::create_dir_all(source_abs.parent().expect("parent")).expect("mkdir");
    std::fs::write(
        &source_abs,
        r#"
use sqlx::query_as as qa;

fn load() {
    let _row = qa!(User, "select 1");
}
"#,
    )
    .expect("write");

    let tree = project_tree(
        vec![
            (
                "",
                dir_entry(&["src"], &["Cargo.toml", "clippy.toml", "guardrail3.toml"]),
            ),
            ("src", dir_entry(&[], &["db.rs"])),
        ],
        vec![
            (
                "Cargo.toml",
                r#"[workspace]
members = []
[workspace.dependencies]
garde = { version = "0.22", features = ["derive"] }
"#,
            ),
            ("clippy.toml", "disallowed-methods = []\ndisallowed-types = []\n"),
            ("guardrail3.toml", "[profile]\nname = \"service\"\n"),
        ],
        root.clone(),
    );

    let results = crate::app::rs::checks::rs::garde::check(&tree);
    assert!(has_result(&results, "RS-GARDE-09", |result| {
        result.severity == Severity::Info
            && result.inventory
            && result.file.as_deref() == Some(source_rel)
    }));

    std::fs::remove_dir_all(root).expect("cleanup");
}
