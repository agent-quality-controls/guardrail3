use guardrail3_app_rs_family_garde_assertions::rs_garde_05_struct_derive_validate as assertions;
use test_support::{dir_entry, project_tree, temp_root};

#[test]
fn handles_multiple_roots() {
    let root = temp_root("rs-garde-05-multi-root");
    let source1_rel = "vendor/lib/src/input.rs";
    let source2_rel = "vendor/tool/src/input.rs";
    let clippy_toml = super::super::canonical_clippy_toml();

    let source1_abs = root.join(source1_rel);
    std::fs::create_dir_all(
        source1_abs
            .parent()
            .expect("fixture source path must have a parent directory"),
    )
    .expect("failed to create fixture source directory");
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
    .expect("failed to write fixture source");

    let source2_abs = root.join(source2_rel);
    std::fs::create_dir_all(
        source2_abs
            .parent()
            .expect("fixture source path must have a parent directory"),
    )
    .expect("failed to create fixture source directory");
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
    .expect("failed to write fixture source");

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

    let results = super::super::run_family(&tree);
    let findings = assertions::findings(&results);
    assert_eq!(
        findings.len(),
        1,
        "unexpected RS-GARDE-05 findings: {findings:#?}"
    );
    assertions::assert_single_error(&results, Some(source1_rel), Some(4), None, None);

    std::fs::remove_dir_all(root).expect("failed to remove temporary fixture root");
}
