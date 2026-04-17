use g3rs_release_config_checks_assertions::rs_release_config_00_publish_must_be_explicit as assertions;

use super::helpers::run_check;

#[test]
fn errors_when_publish_is_missing() {
    let results = run_check(
        r#"
[package]
name = "demo"
version = "0.1.0"
edition = "2024"
"#,
        None,
    );

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "demo: publish must be explicit",
            "Crate `demo` does not set `[package].publish`. Add `publish = true` if this crate publishes or `publish = false` if it does not.",
            "Cargo.toml",
            false,
        )],
    );
}

#[test]
fn stands_down_when_publish_is_false() {
    let results = run_check(
        r#"
[package]
name = "demo"
version = "0.1.0"
edition = "2024"
publish = false
"#,
        None,
    );

    assertions::assert_no_findings(&results);
}

#[test]
fn stands_down_when_publish_is_inherited() {
    let results = run_check(
        r#"
[package]
name = "demo"
version.workspace = true
edition = "2024"
publish.workspace = true
"#,
        Some(
            r#"
[workspace.package]
version = "0.1.0"
publish = false
"#,
        ),
    );

    assertions::assert_no_findings(&results);
}
