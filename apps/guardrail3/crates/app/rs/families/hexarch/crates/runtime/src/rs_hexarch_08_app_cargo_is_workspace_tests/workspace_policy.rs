use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_08_app_cargo_is_workspace as assertions;
use super::{copy_fixture, write_file};

fn rewrite_as_package_manifest(app_name: &str, original: &str) -> String {
    let parsed = toml::from_str::<toml::Value>(original).expect("parse original workspace cargo");
    let workspace = parsed.get("workspace").expect("workspace table");
    let workspace_package = workspace
        .get("package")
        .expect("workspace.package table for fixture");

    let version = workspace_package
        .get("version")
        .and_then(toml::Value::as_str)
        .expect("workspace.package.version");
    let edition = workspace_package
        .get("edition")
        .and_then(toml::Value::as_str)
        .expect("workspace.package.edition");
    let publish = workspace_package
        .get("publish")
        .and_then(toml::Value::as_bool)
        .expect("workspace.package.publish");

    let mut manifest = format!(
        "[package]\nname = \"{app_name}\"\nversion = \"{version}\"\nedition = \"{edition}\"\npublish = {publish}\n"
    );

    if let Some(rust_lints) = workspace.get("lints").and_then(|lints| lints.get("rust")) {
        manifest.push_str("\n[lints.rust]\n");
        manifest.push_str(&toml::to_string(rust_lints).expect("serialize rust lints"));
    }

    if let Some(clippy_lints) = workspace.get("lints").and_then(|lints| lints.get("clippy")) {
        manifest.push_str("\n[lints.clippy]\n");
        manifest.push_str(&toml::to_string(clippy_lints).expect("serialize clippy lints"));
    }

    manifest
}

#[test]
fn package_style_app_cargo_hits_every_mutated_app() {
    let tmp = copy_fixture();
    for app in ["devctl", "backend", "worker"] {
        let rel = format!("apps/{app}/Cargo.toml");
        let original =
            std::fs::read_to_string(tmp.path().join(&rel)).expect("read original workspace cargo");
        write_file(
            tmp.path(),
            &rel,
            &rewrite_as_package_manifest(app, &original),
        );
    }

    let results = super::run_family(tmp.path());
    assertions::assert_expected_rule_results(
        &results,
        &[
            assertions::ExpectedRuleResult {
                file: Some("apps/devctl/Cargo.toml"),
                file_contains: None,
                title_contains: Some(&["must be a workspace"]),
                message_contains: None,
            },
            assertions::ExpectedRuleResult {
                file: Some("apps/backend/Cargo.toml"),
                file_contains: None,
                title_contains: Some(&["must be a workspace"]),
                message_contains: None,
            },
            assertions::ExpectedRuleResult {
                file: Some("apps/worker/Cargo.toml"),
                file_contains: None,
                title_contains: Some(&["must be a workspace"]),
                message_contains: None,
            },
        ],
    );
}

#[test]
fn single_package_style_app_cargo_hits_only_that_app() {
    let tmp = copy_fixture();
    let original = std::fs::read_to_string(tmp.path().join("apps/devctl/Cargo.toml"))
        .expect("read original devctl workspace cargo");
    write_file(
        tmp.path(),
        "apps/devctl/Cargo.toml",
        &rewrite_as_package_manifest("devctl", &original),
    );

    let results = super::run_family(tmp.path());
    assertions::assert_expected_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            file: Some("apps/devctl/Cargo.toml"),
            file_contains: None,
            title_contains: Some(&["must be a workspace"]),
            message_contains: None,
        }],
    );
}
