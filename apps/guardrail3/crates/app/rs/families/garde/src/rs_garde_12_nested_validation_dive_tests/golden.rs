use crate::test_support::{canonical_clippy_toml, dir_entry, project_tree, temp_root};

#[test]
fn stays_quiet_when_nested_validated_fields_use_dive() {
    let root = temp_root("rs-garde-12-golden");
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
struct Payload {
    #[garde(length(min = 1))]
    title: String,
}

#[derive(Deserialize, Validate)]
struct Input {
    #[garde(dive)]
    payload: Payload,
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
        .filter(|result| result.id == "RS-GARDE-12")
        .collect();
    assert!(results.is_empty());

    std::fs::remove_dir_all(root).expect("cleanup");
}
