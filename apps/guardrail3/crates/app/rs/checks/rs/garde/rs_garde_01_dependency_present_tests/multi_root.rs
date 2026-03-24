use crate::domain::report::Severity;

use super::super::super::test_support::{dir_entry, project_tree, temp_root};

#[test]
fn evaluates_workspace_and_standalone_package_roots() {
    let root = temp_root("rs-garde-01-multi");

    let tree = project_tree(
        vec![
            (
                "",
                dir_entry(
                    &["vendor"],
                    &["Cargo.toml", "clippy.toml", "guardrail3.toml"],
                ),
            ),
            ("vendor", dir_entry(&["lib", "tool"], &[])),
            (
                "vendor/lib",
                dir_entry(&["src"], &["Cargo.toml", "clippy.toml"]),
            ),
            ("vendor/lib/src", dir_entry(&[], &["lib.rs"])),
            (
                "vendor/tool",
                dir_entry(&["src"], &["Cargo.toml", "clippy.toml"]),
            ),
            ("vendor/tool/src", dir_entry(&[], &["main.rs"])),
        ],
        vec![
            (
                "Cargo.toml",
                r#"[workspace]
members = []
[workspace.dependencies]
serde = { version = "1", features = ["derive"] }
"#,
            ),
            (
                "clippy.toml",
                "disallowed-methods = []\ndisallowed-types = []\n",
            ),
            ("guardrail3.toml", "[profile]\nname = \"service\"\n"),
            (
                "vendor/lib/Cargo.toml",
                r#"[package]
name = "lib"
version = "0.1.0"
[dependencies]
garde = "0.22"
"#,
            ),
            (
                "vendor/lib/clippy.toml",
                "disallowed-methods = []\ndisallowed-types = []\n",
            ),
            ("vendor/lib/src/lib.rs", "pub fn ok() {}"),
            (
                "vendor/tool/Cargo.toml",
                r#"[package]
name = "tool"
version = "0.1.0"
[dependencies]
serde = "1"
"#,
            ),
            (
                "vendor/tool/clippy.toml",
                "disallowed-methods = []\ndisallowed-types = []\n",
            ),
            ("vendor/tool/src/main.rs", "fn main() {}"),
        ],
        root.clone(),
    );

    let mut rs_garde_01_results: Vec<_> = crate::app::rs::checks::rs::garde::check(&tree)
        .into_iter()
        .filter(|r| r.id == "RS-GARDE-01")
        .collect();

    rs_garde_01_results.sort_by_key(|r| r.file.clone());

    assert_eq!(rs_garde_01_results.len(), 3);

    assert_eq!(rs_garde_01_results[0].severity, Severity::Error);
    assert_eq!(rs_garde_01_results[0].file.as_deref(), Some("Cargo.toml"));
    assert!(!rs_garde_01_results[0].inventory);

    assert_eq!(rs_garde_01_results[1].severity, Severity::Info);
    assert_eq!(
        rs_garde_01_results[1].file.as_deref(),
        Some("vendor/lib/Cargo.toml")
    );
    assert!(rs_garde_01_results[1].inventory);

    assert_eq!(rs_garde_01_results[2].severity, Severity::Error);
    assert_eq!(
        rs_garde_01_results[2].file.as_deref(),
        Some("vendor/tool/Cargo.toml")
    );
    assert!(!rs_garde_01_results[2].inventory);

    std::fs::remove_dir_all(root).expect("cleanup");
}
