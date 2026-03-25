use crate::test_support::{
    canonical_clippy_toml, dir_entry, project_tree, temp_root,
};
use guardrail3_domain_report::Severity;

#[test]
fn handles_multiple_roots() {
    let root = temp_root("rs-garde-05-multi-root");
    let source1_rel = "vendor/lib/src/input.rs";
    let source2_rel = "vendor/tool/src/input.rs";
    let clippy_toml = canonical_clippy_toml();

    let source1_abs = root.join(source1_rel);
    std::fs::create_dir_all(source1_abs.parent().expect("parent")).expect("mkdir");
    std::fs::write(
        &source1_abs,
        r#"
use serde::Deserialize;

#[derive(Deserialize)]
struct InputA {
    name: String,
}
"#,
    )
    .expect("write");

    let source2_abs = root.join(source2_rel);
    std::fs::create_dir_all(source2_abs.parent().expect("parent")).expect("mkdir");
    std::fs::write(
        &source2_abs,
        r#"
use serde::Deserialize;
use garde::Validate;

#[derive(Deserialize, Validate)]
struct InputB {
    name: String,
}
"#,
    )
    .expect("write");

    let tree = project_tree(
        vec![
            ("", dir_entry(&["vendor"], &["Cargo.toml"])),
            ("vendor", dir_entry(&["lib", "tool"], &[])),
            (
                "vendor/lib",
                dir_entry(&["src"], &["Cargo.toml", "clippy.toml", "guardrail3.toml"]),
            ),
            ("vendor/lib/src", dir_entry(&[], &["input.rs"])),
            (
                "vendor/tool",
                dir_entry(&["src"], &["Cargo.toml", "clippy.toml", "guardrail3.toml"]),
            ),
            ("vendor/tool/src", dir_entry(&[], &["input.rs"])),
        ],
        vec![
            ("Cargo.toml", "[workspace]\nmembers = []\n"),
            (
                "vendor/lib/Cargo.toml",
                r#"[package]
name = "lib"
[dependencies]
garde = { version = "0.22", features = ["derive"] }
"#,
            ),
            ("vendor/lib/clippy.toml", clippy_toml.as_str()),
            (
                "vendor/lib/guardrail3.toml",
                "[profile]\nname = \"service\"\n",
            ),
            (
                "vendor/tool/Cargo.toml",
                r#"[package]
name = "tool"
[dependencies]
garde = { version = "0.22", features = ["derive"] }
"#,
            ),
            ("vendor/tool/clippy.toml", clippy_toml.as_str()),
            (
                "vendor/tool/guardrail3.toml",
                "[profile]\nname = \"service\"\n",
            ),
        ],
        root.clone(),
    );

    let mut results: Vec<_> = crate::check(&tree, None)
        .into_iter()
        .filter(|result| result.id == "RS-GARDE-05")
        .collect();
    results.sort_by(|a, b| a.file.cmp(&b.file));

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(results[0].file.as_deref(), Some(source1_rel));
    assert_eq!(results[0].line, Some(4));

    std::fs::remove_dir_all(root).expect("cleanup");
}
