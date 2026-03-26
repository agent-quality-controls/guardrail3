use crate::test_support::{canonical_clippy_toml, dir_entry, project_tree, temp_root};
use guardrail3_domain_report::{CheckResult, Severity};

fn run_enum_boundary(source: &str) -> Vec<CheckResult> {
    let root = temp_root("rs-garde-08-derive-variants");
    let source_abs = root.join("src/input.rs");
    let clippy_toml = canonical_clippy_toml();
    std::fs::create_dir_all(source_abs.parent().expect("parent")).expect("mkdir");
    std::fs::write(&source_abs, source).expect("write");

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
        .filter(|result| result.id == "RS-GARDE-08")
        .collect();

    std::fs::remove_dir_all(root).expect("cleanup");
    results
}

#[test]
fn errors_when_parser_enum_lacks_validate() {
    let results = run_enum_boundary(
        r#"
use clap::Parser;

#[derive(Parser)]
enum Input {
    Variant(String),
}
"#,
    );

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(
        results[0].message,
        "Enum `Input` derives Parser and has non-primitive payload fields, but does not derive `Validate`."
    );
}

#[test]
fn errors_when_args_enum_lacks_validate() {
    let results = run_enum_boundary(
        r#"
use clap::Args;

#[derive(Args)]
enum Input {
    Variant(String),
}
"#,
    );

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(
        results[0].message,
        "Enum `Input` derives Args and has non-primitive payload fields, but does not derive `Validate`."
    );
}

#[test]
fn errors_when_from_row_enum_lacks_validate() {
    let results = run_enum_boundary(
        r#"
use sqlx::FromRow;

#[derive(FromRow)]
enum Input {
    Variant(String),
}
"#,
    );

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(
        results[0].message,
        "Enum `Input` derives FromRow and has non-primitive payload fields, but does not derive `Validate`."
    );
}
