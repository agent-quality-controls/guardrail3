use super::{
    FULL_CLIPPY_LINTS, FULL_RUST_LINTS, FULL_STANDALONE_CLIPPY_LINTS, FULL_STANDALONE_RUST_LINTS,
    check_results, entry, rule_results, tree,
};

#[test]
fn supports_workspace_and_standalone_policy_roots_in_one_repo() {
    let workspace_manifest = format!(
        r#"
            [workspace]
            members = ["crates/api"]
            resolver = "2"

            [workspace.package]
            edition = "2024"
            rust-version = "1.85"

            {FULL_RUST_LINTS}
            {FULL_CLIPPY_LINTS}
        "#
    );
    let standalone_manifest = format!(
        r#"
            [package]
            name = "helper"
            edition = "2024"
            rust-version = "1.85"

            {FULL_STANDALONE_RUST_LINTS}
            {FULL_STANDALONE_CLIPPY_LINTS}
        "#
    );

    let results = check_results(&tree(
        &[
            ("", entry(&["crates", "tools"], &["Cargo.toml"])),
            ("crates", entry(&["api"], &[])),
            ("crates/api", entry(&[], &["Cargo.toml"])),
            ("tools", entry(&["helper"], &[])),
            ("tools/helper", entry(&[], &["Cargo.toml"])),
        ],
        &[
            ("Cargo.toml", &workspace_manifest),
            (
                "crates/api/Cargo.toml",
                r#"
                    [package]
                    name = "api"
                    edition = "2024"

                    [lints]
                    workspace = true
                "#,
            ),
            ("tools/helper/Cargo.toml", &standalone_manifest),
        ],
    ));

    let rule = rule_results(&results, "RS-CARGO-01");
    assert_eq!(rule.len(), 2, "unexpected results: {rule:#?}");
    assert!(rule.iter().all(|result| result.inventory));
    assert!(
        rule.iter()
            .any(|result| result.file.as_deref() == Some("Cargo.toml"))
    );
    assert!(
        rule.iter()
            .any(|result| result.file.as_deref() == Some("tools/helper/Cargo.toml"))
    );
}

#[test]
fn local_library_profile_requires_unreachable_pub() {
    let standalone_manifest = format!(
        r#"
            [package]
            name = "helper"
            edition = "2024"

            {FULL_STANDALONE_RUST_LINTS}
            {FULL_STANDALONE_CLIPPY_LINTS}
        "#
    );
    let standalone_manifest = standalone_manifest.replace(r#"unreachable_pub = "deny""#, "");

    let results = check_results(&tree(
        &[("pkg", entry(&[], &["Cargo.toml", "guardrail3.toml"]))],
        &[
            ("pkg/Cargo.toml", &standalone_manifest),
            ("pkg/guardrail3.toml", "[profile]\nname = \"library\"\n"),
        ],
    ));

    let rule = rule_results(&results, "RS-CARGO-01");
    assert_eq!(rule.len(), 1, "unexpected results: {rule:#?}");
    let result = rule[0];
    assert_eq!(result.file.as_deref(), Some("pkg/Cargo.toml"));
    assert_eq!(result.title, "missing library rust lint `unreachable_pub`");
}
