use crate::test_support::{canonical_clippy_toml, dir_entry, project_tree, temp_root};
use guardrail3_domain_report::{CheckResult, Severity};

fn run_struct_boundary(source: &str) -> Vec<CheckResult> {
    let root = temp_root("rs-garde-05-derive-variants");
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
        .filter(|result| result.id == "RS-GARDE-05")
        .collect();

    std::fs::remove_dir_all(root).expect("cleanup");
    results
}

#[test]
fn errors_when_parser_struct_lacks_validate() {
    let results = run_struct_boundary(
        r#"
use clap::Parser;

#[derive(Parser)]
struct Input {
    name: String,
}
"#,
    );

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(
        results[0].message,
        "Struct `Input` derives Parser but does not derive `Validate`. Non-primitive input boundary structs must derive garde validation."
    );
}

#[test]
fn errors_when_args_struct_lacks_validate() {
    let results = run_struct_boundary(
        r#"
use clap::Args;

#[derive(Args)]
struct Input {
    name: String,
}
"#,
    );

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(
        results[0].message,
        "Struct `Input` derives Args but does not derive `Validate`. Non-primitive input boundary structs must derive garde validation."
    );
}

#[test]
fn errors_when_from_row_struct_lacks_validate() {
    let results = run_struct_boundary(
        r#"
use sqlx::FromRow;

#[derive(FromRow)]
struct Input {
    name: String,
}
"#,
    );

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(
        results[0].message,
        "Struct `Input` derives FromRow but does not derive `Validate`. Non-primitive input boundary structs must derive garde validation."
    );
}
