use guardrail3_app_rs_family_garde_assertions::rs_garde_05_struct_derive_validate as assertions;
use test_support::{dir_entry, project_tree, temp_root};

#[test]
fn skips_primitive_only_struct_boundary_without_validate() {
    let root = temp_root("rs-garde-05-primitive-only");
    let source_abs = root.join("src/input.rs");
    let clippy_toml = super::super::canonical_clippy_toml();
    std::fs::create_dir_all(source_abs.parent().expect("parent")).expect("mkdir");
    std::fs::write(
        &source_abs,
        r#"
use serde::Deserialize;

#[derive(Deserialize)]
struct Input {
    count: u64,
    ok: bool,
    initial: char,
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

    let results = super::super::run_family(&tree);
    let _ = assertions::findings(&results);
    assertions::assert_rule_quiet(&results);

    std::fs::remove_dir_all(root).expect("cleanup");
}

#[test]
fn skips_primitive_array_struct_boundary_without_validate() {
    let root = temp_root("rs-garde-05-primitive-array");
    let source_abs = root.join("src/input.rs");
    let clippy_toml = super::super::canonical_clippy_toml();
    std::fs::create_dir_all(source_abs.parent().expect("parent")).expect("mkdir");
    std::fs::write(
        &source_abs,
        r#"
use serde::Deserialize;

#[derive(Deserialize)]
struct Input {
    digest: [u8; 32],
    matrix: [[i16; 4]; 2],
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

    let results = super::super::run_family(&tree);
    let _ = assertions::findings(&results);
    assertions::assert_rule_quiet(&results);

    std::fs::remove_dir_all(root).expect("cleanup");
}
