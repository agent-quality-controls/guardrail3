use crate::domain::report::Severity;

use super::super::inputs::ManualDeserializeImplInput;
use super::super::test_support::{dir_entry, has_result, manual_impl, project_tree, temp_root};
use super::check;

#[test]
fn errors_when_manual_deserialize_impl_needs_validate() {
    let mut results = Vec::new();
    check(
        &ManualDeserializeImplInput::new(&manual_impl(true, false)),
        &mut results,
    );
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-GARDE-07");
    assert_eq!(results[0].severity, Severity::Error);
}

#[test]
fn skips_manual_deserialize_impl_when_validate_present() {
    let mut results = Vec::new();
    check(
        &ManualDeserializeImplInput::new(&manual_impl(true, true)),
        &mut results,
    );
    assert!(results.is_empty());
}

#[test]
fn family_flags_manual_deserialize_impl_without_validate() {
    let root = temp_root("rs-garde-07-family");
    let source_rel = "src/input.rs";
    let source_abs = root.join(source_rel);
    std::fs::create_dir_all(source_abs.parent().expect("parent")).expect("mkdir");
    std::fs::write(
        &source_abs,
        r#"
use serde::Deserialize;

struct Input {
    name: String,
}

impl<'de> Deserialize<'de> for Input {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        todo!()
    }
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
    assert!(has_result(&results, "RS-GARDE-07", |result| {
        result.severity == Severity::Error && result.file.as_deref() == Some(source_rel)
    }));

    std::fs::remove_dir_all(root).expect("cleanup");
}

#[test]
fn family_flags_aliased_deserialize_impl_without_validate() {
    let root = temp_root("rs-garde-07-aliased-family");
    let source_rel = "src/input.rs";
    let source_abs = root.join(source_rel);
    std::fs::create_dir_all(source_abs.parent().expect("parent")).expect("mkdir");
    std::fs::write(
        &source_abs,
        r#"
use serde::Deserialize as De;

struct Input {
    name: String,
}

impl<'de> De<'de> for Input {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        todo!()
    }
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
    assert!(has_result(&results, "RS-GARDE-07", |result| {
        result.severity == Severity::Error && result.file.as_deref() == Some(source_rel)
    }));

    std::fs::remove_dir_all(root).expect("cleanup");
}
