use crate::domain::report::Severity;

use super::super::super::test_support::{dir_entry, project_tree, temp_root};

#[test]
fn errors_when_garde_dependency_missing() {
    let root = temp_root("rs-garde-01-missing");

    let tree = project_tree(
        vec![
            (
                "",
                dir_entry(&["src"], &["Cargo.toml", "clippy.toml", "guardrail3.toml"]),
            ),
            ("src", dir_entry(&[], &["main.rs"])),
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
            ("src/main.rs", "fn main() {}"),
        ],
        root.clone(),
    );

    let results = crate::app::rs::checks::rs::garde::check(&tree);

    let rs_garde_01_results: Vec<_> = results
        .into_iter()
        .filter(|r| r.id == "RS-GARDE-01")
        .collect();

    assert_eq!(rs_garde_01_results.len(), 1);
    assert_eq!(rs_garde_01_results[0].severity, Severity::Error);
    assert_eq!(rs_garde_01_results[0].file.as_deref(), Some("Cargo.toml"));
    assert!(!rs_garde_01_results[0].inventory);
    assert_eq!(rs_garde_01_results[0].title, "garde dependency missing");
    assert_eq!(
        rs_garde_01_results[0].message,
        "Missing `garde` dependency in `Cargo.toml` for this workspace root. Runtime input validation at Rust adapter boundaries requires garde."
    );

    std::fs::remove_dir_all(root).expect("cleanup");
}
