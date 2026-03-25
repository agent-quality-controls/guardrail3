use crate::test_support::{
    canonical_clippy_toml, dir_entry, project_tree, temp_root,
};

#[test]
fn ignores_primitive_unvalidatable_and_nested_dive_fields() {
    let root = temp_root("rs-garde-11-false-positives");
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
struct Metadata {
    #[garde(length(min = 1))]
    tags: Vec<String>,
}

#[derive(Deserialize, Validate)]
struct Input {
    count: usize,
    tags: std::collections::HashMap<String, String>,
    #[garde(dive)]
    metadata: Metadata,
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

    let results: Vec<_> = crate::check(&tree, None)
        .into_iter()
        .filter(|result| result.id == "RS-GARDE-11")
        .collect();
    assert!(results.is_empty());

    std::fs::remove_dir_all(root).expect("cleanup");
}
