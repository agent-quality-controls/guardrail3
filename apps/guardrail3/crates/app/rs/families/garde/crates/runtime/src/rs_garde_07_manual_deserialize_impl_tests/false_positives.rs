use guardrail3_app_rs_family_garde_assertions::rs_garde_07_manual_deserialize_impl as assertions;
use test_support::{dir_entry, project_tree, temp_root};

#[test]
fn skips_manual_deserialize_impl_when_validate_present() {
    let root = temp_root("rs-garde-07-false-pos");
    let source_rel = "src/input.rs";
    let source_abs = root.join(source_rel);
    let clippy_toml = super::super::canonical_clippy_toml();
    std::fs::create_dir_all(source_abs.parent().expect("fixture source path must have a parent directory")).expect("failed to create fixture source directory");
    std::fs::write(
        &source_abs,
        r#"
use serde::Deserialize;
use garde::Validate;

#[derive(Validate)]
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
    .expect("failed to write fixture source");

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
    let findings = assertions::findings(&results);
    assert!(findings.is_empty());
    assertions::assert_rule_quiet(&results);

    std::fs::remove_dir_all(root).expect("failed to remove temporary fixture root");
}

#[test]
fn ignores_non_serde_deserialize_trait_with_same_name() {
    let root = temp_root("rs-garde-07-non-serde-deserialize");
    let source_rel = "src/input.rs";
    let source_abs = root.join(source_rel);
    let clippy_toml = super::super::canonical_clippy_toml();
    std::fs::create_dir_all(source_abs.parent().expect("fixture source path must have a parent directory")).expect("failed to create fixture source directory");
    std::fs::write(
        &source_abs,
        r#"
mod fake {
    pub trait Deserialize<'de> {}
}

use fake::Deserialize;

struct Input {
    name: String,
}

impl<'de> Deserialize<'de> for Input {}
"#,
    )
    .expect("failed to write fixture source");

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
    let findings = assertions::findings(&results);
    assert!(findings.is_empty());
    assertions::assert_rule_quiet(&results);

    std::fs::remove_dir_all(root).expect("failed to remove temporary fixture root");
}
