use crate::domain::report::Severity;

use super::super::inputs::DerivedBoundaryTypeInput;
use super::super::test_support::{derived_target, dir_entry, has_result, project_tree, temp_root};
use super::check;
use crate::app::rs::checks::rs::garde::parse::BoundaryKind;

#[test]
fn errors_when_enum_boundary_type_lacks_validate() {
    let mut results = Vec::new();
    check(
        &DerivedBoundaryTypeInput::new(&derived_target(BoundaryKind::Enum, false)),
        &mut results,
    );
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-GARDE-08");
    assert_eq!(results[0].severity, Severity::Error);
}

#[test]
fn skips_validated_enum_boundary_types() {
    let mut results = Vec::new();
    check(
        &DerivedBoundaryTypeInput::new(&derived_target(BoundaryKind::Enum, true)),
        &mut results,
    );
    assert!(results.is_empty());
}

#[test]
fn family_does_not_flag_c_like_enums() {
    let root = temp_root("rs-garde-08-c-like-enum");
    let source_rel = "src/input.rs";
    let source_abs = root.join(source_rel);
    std::fs::create_dir_all(source_abs.parent().expect("parent")).expect("mkdir");
    std::fs::write(
        &source_abs,
        r#"
use serde::Deserialize;

#[derive(Deserialize)]
enum Color {
    Red,
    Green,
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
    assert!(!has_result(&results, "RS-GARDE-08", |result| {
        result.file.as_deref() == Some(source_rel)
    }));

    std::fs::remove_dir_all(root).expect("cleanup");
}
