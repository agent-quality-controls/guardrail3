use super::{collected_facts, dir_entry, project_tree};
use guardrail3_app_rs_family_deps_assertions::rs_deps_09_cargo_lock_present as assertions;

#[test]
fn implicit_workspace_path_members_do_not_require_own_lockfiles() {
    let tree = project_tree(
        vec![
            ("", dir_entry(&["crates"], &["Cargo.toml", "Cargo.lock"])),
            ("crates", dir_entry(&["runtime"], &[])),
            (
                "crates/runtime",
                dir_entry(&["assertions"], &["Cargo.toml"]),
            ),
            ("crates/runtime/assertions", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![
            (
                "Cargo.toml",
                r#"
                    [workspace]
                    members = ["crates/runtime"]
                "#,
            ),
            (
                "crates/runtime/Cargo.toml",
                r#"
                    [package]
                    name = "runtime"

                    [dev-dependencies]
                    runtime_assertions = { path = "assertions" }
                "#,
            ),
            (
                "crates/runtime/assertions/Cargo.toml",
                r#"
                    [package]
                    name = "runtime-assertions"
                "#,
            ),
        ],
    );

    let facts = collected_facts(&tree, &[]);
    let results = super::run_with_facts(&facts);

    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            file: Some("Cargo.lock"),
            severity: Some(assertions::Severity::Info),
            message: Some("Rust root `.` has `Cargo.lock` committed."),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}
