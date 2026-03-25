use crate::test_support::{
    canonical_clippy_toml, dir_entry, project_tree, temp_root,
};
use guardrail3_domain_report::Severity;

#[test]
fn reports_only_owned_root_enum_boundary_gap() {
    let root = temp_root("rs-garde-08-multi-root");
    let clippy_toml = canonical_clippy_toml();

    for (rel, source) in [
        (
            "vendor/lib/src/input.rs",
            r#"
use serde::Deserialize;

#[derive(Deserialize)]
enum InputA {
    Variant(String),
}
"#,
        ),
        (
            "vendor/tool/src/input.rs",
            r#"
use serde::Deserialize;
use garde::Validate;

#[derive(Deserialize, Validate)]
enum InputB {
    Variant(String),
}
"#,
        ),
    ] {
        let abs = root.join(rel);
        std::fs::create_dir_all(abs.parent().expect("parent")).expect("mkdir");
        std::fs::write(abs, source).expect("write");
    }

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

    let results: Vec<_> = crate::check(&tree, None)
        .into_iter()
        .filter(|result| result.id == "RS-GARDE-08")
        .collect();

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(results[0].file.as_deref(), Some("vendor/lib/src/input.rs"));

    std::fs::remove_dir_all(root).expect("cleanup");
}
