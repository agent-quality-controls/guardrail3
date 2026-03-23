use crate::domain::report::Severity;

use super::super::inputs::GardeInputFailureInput;
use super::super::test_support::{dir_entry, has_result, input_failure, project_tree, temp_root};
use super::check;

#[test]
fn errors_on_input_failure() {
    let mut results = Vec::new();
    check(
        &GardeInputFailureInput::new(&input_failure(
            "src/lib.rs",
            "Failed to parse Rust source file for garde checks: unexpected token",
        )),
        &mut results,
    );
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-GARDE-10");
    assert_eq!(results[0].severity, Severity::Error);
}

#[test]
fn family_surfaces_source_parse_failures() {
    let root = temp_root("rs-garde-10-parse-failure");
    let source_rel = "src/input.rs";
    let source_abs = root.join(source_rel);
    std::fs::create_dir_all(source_abs.parent().expect("parent")).expect("mkdir");
    std::fs::write(&source_abs, "fn broken( {").expect("write");

    let tree = project_tree(
        vec![
            (
                "",
                dir_entry(&["src"], &["Cargo.toml", "clippy.toml", "guardrail3.toml"]),
            ),
            ("src", dir_entry(&[], &["input.rs"])),
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
            (
                "clippy.toml",
                "disallowed-methods = []\ndisallowed-types = []\n",
            ),
            ("guardrail3.toml", "[profile]\nname = \"service\"\n"),
        ],
        root.clone(),
    );

    let results = crate::app::rs::checks::rs::garde::check(&tree);
    assert!(has_result(&results, "RS-GARDE-10", |result| {
        result.severity == Severity::Error
            && result.file.as_deref() == Some(source_rel)
            && result
                .message
                .contains("Failed to parse Rust source file for garde checks")
    }));

    std::fs::remove_dir_all(root).expect("cleanup");
}
