use crate::test_fixtures::canonical_clippy_toml;
use test_support::{dir_entry, project_tree, temp_root};

#[test]
fn skips_validated_struct_boundary_types() {
    let root = temp_root("rs-garde-05-false-pos");
    let source_rel = "src/input.rs";
    let source_abs = root.join(source_rel);
    let clippy_toml = canonical_clippy_toml();
    std::fs::create_dir_all(source_abs.parent().expect("parent")).expect("mkdir");
    std::fs::write(
        &source_abs,
        r#"
use serde::Deserialize;
use garde::Validate;

#[derive(Deserialize, Validate)]
struct Input {
    name: String,
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
            ("clippy.toml", clippy_toml.as_str()),
            ("guardrail3.toml", "[profile]\nname = \"service\"\n"),
        ],
        root.clone(),
    );

    let results: Vec<_> = crate::test_fixtures::run_family(&tree)
        .into_iter()
        .filter(|result| result.id == "RS-GARDE-05")
        .collect();
    assert!(results.is_empty());

    std::fs::remove_dir_all(root).expect("cleanup");
}

#[test]
fn ignores_non_boundary_derive_with_deserialize_suffix() {
    let root = temp_root("rs-garde-05-fake-deserialize");
    let source_rel = "src/input.rs";
    let source_abs = root.join(source_rel);
    let clippy_toml = canonical_clippy_toml();
    std::fs::create_dir_all(source_abs.parent().expect("parent")).expect("mkdir");
    std::fs::write(
        &source_abs,
        r#"
#[derive(fake::Deserialize)]
struct Input {
    name: String,
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
            ("clippy.toml", clippy_toml.as_str()),
            ("guardrail3.toml", "[profile]\nname = \"service\"\n"),
        ],
        root.clone(),
    );

    let results: Vec<_> = crate::test_fixtures::run_family(&tree)
        .into_iter()
        .filter(|result| result.id == "RS-GARDE-05")
        .collect();
    assert!(
        results.is_empty(),
        "non-canonical derive macros should not create boundary inventory"
    );

    std::fs::remove_dir_all(root).expect("cleanup");
}

#[test]
fn skips_struct_with_aliased_validate_derive() {
    let root = temp_root("rs-garde-05-aliased-validate");
    let source_rel = "src/input.rs";
    let source_abs = root.join(source_rel);
    let clippy_toml = canonical_clippy_toml();
    std::fs::create_dir_all(source_abs.parent().expect("parent")).expect("mkdir");
    std::fs::write(
        &source_abs,
        r#"
use serde::Deserialize as De;
use garde::Validate as V;

#[derive(De, V)]
struct Input {
    name: String,
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
            ("clippy.toml", clippy_toml.as_str()),
            ("guardrail3.toml", "[profile]\nname = \"service\"\n"),
        ],
        root.clone(),
    );

    let results: Vec<_> = crate::test_fixtures::run_family(&tree)
        .into_iter()
        .filter(|result| result.id == "RS-GARDE-05")
        .collect();
    assert!(
        results.is_empty(),
        "aliased derive macros should still count"
    );

    std::fs::remove_dir_all(root).expect("cleanup");
}
