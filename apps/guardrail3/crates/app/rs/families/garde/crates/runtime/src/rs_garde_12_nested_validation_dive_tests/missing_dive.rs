use crate::test_fixtures::canonical_clippy_toml;
use guardrail3_domain_report::Severity;
use test_support::{dir_entry, project_tree, temp_root};

#[test]
fn errors_when_nested_validated_field_lacks_dive() {
    let root = temp_root("rs-garde-12-missing");
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

    let results: Vec<_> = crate::test_fixtures::run_family(&tree)
        .into_iter()
        .filter(|result| result.id == "RS-GARDE-12")
        .collect();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(results[0].file.as_deref(), Some(source_rel));
    assert_eq!(results[0].line, Some(13));
    assert_eq!(
        results[0].title,
        "nested validated field `payload` missing garde(dive)"
    );
    assert_eq!(
        results[0].message,
        "Field `payload` in validated boundary `Input` points at validated nested type `Payload` but is missing `#[garde(dive)]`. Nested validated fields must opt into recursive garde validation."
    );

    std::fs::remove_dir_all(root).expect("cleanup");
}

#[test]
fn errors_when_array_of_nested_validated_field_lacks_dive() {
    let root = temp_root("rs-garde-12-array-missing");
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
    payloads: [Payload; 2],
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

    let results = crate::test_fixtures::run_family(&tree);
    let rs_garde_12_results: Vec<_> = results
        .iter()
        .filter(|result| result.id == "RS-GARDE-12")
        .collect();
    assert_eq!(rs_garde_12_results.len(), 1);
    assert_eq!(rs_garde_12_results[0].severity, Severity::Error);
    assert_eq!(rs_garde_12_results[0].file.as_deref(), Some(source_rel));
    assert_eq!(rs_garde_12_results[0].line, Some(13));
    assert_eq!(
        rs_garde_12_results[0].title,
        "nested validated field `payloads` missing garde(dive)"
    );
    assert!(
        results.iter().all(|result| result.id != "RS-GARDE-11"),
        "array of nested validated types should route to RS-GARDE-12, not RS-GARDE-11: {results:#?}"
    );

    std::fs::remove_dir_all(root).expect("cleanup");
}
