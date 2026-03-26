use crate::test_support::{canonical_clippy_toml, dir_entry, project_tree, temp_root};
use guardrail3_domain_report::Severity;

#[test]
fn local_missing_dive_only_errors_for_owned_root() {
    let root = temp_root("rs-garde-12-multi-root");
    let local_rel = "vendor/lib/src/input.rs";
    let shared_rel = "src/input.rs";
    let clippy_toml = canonical_clippy_toml();
    let local_abs = root.join(local_rel);
    let shared_abs = root.join(shared_rel);
    std::fs::create_dir_all(local_abs.parent().expect("parent")).expect("mkdir local");
    std::fs::create_dir_all(shared_abs.parent().expect("parent")).expect("mkdir shared");
    std::fs::write(
        &shared_abs,
        r#"
use serde::Deserialize;
use garde::Validate;

#[derive(Deserialize, Validate)]
struct SharedPayload {
    #[garde(length(min = 1))]
    title: String,
}

#[derive(Deserialize, Validate)]
struct SharedInput {
    #[garde(dive)]
    payload: SharedPayload,
}
"#,
    )
    .expect("write shared");
    std::fs::write(
        &local_abs,
        r#"
use serde::Deserialize;
use garde::Validate;

#[derive(Deserialize, Validate)]
struct LocalPayload {
    #[garde(length(min = 1))]
    title: String,
}

#[derive(Deserialize, Validate)]
struct LocalInput {
    payload: LocalPayload,
}
"#,
    )
    .expect("write local");

    let tree = project_tree(
        vec![
            (
                "",
                dir_entry(&["src", "vendor"], &["Cargo.toml", "clippy.toml"]),
            ),
            ("src", dir_entry(&[], &["input.rs"])),
            ("vendor", dir_entry(&["lib"], &[])),
            (
                "vendor/lib",
                dir_entry(&["src"], &["Cargo.toml", "clippy.toml", "guardrail3.toml"]),
            ),
            ("vendor/lib/src", dir_entry(&[], &["input.rs"])),
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
        ],
        root.clone(),
    );

    let results: Vec<_> = crate::check(&tree, None)
        .into_iter()
        .filter(|result| result.id == "RS-GARDE-12")
        .collect();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(results[0].file.as_deref(), Some(local_rel));

    std::fs::remove_dir_all(root).expect("cleanup");
}
