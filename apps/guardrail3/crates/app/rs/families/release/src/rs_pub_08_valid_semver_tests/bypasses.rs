use super::super::super::check as run_family;
use super::super::super::test_support::{StubToolChecker, dir_entry, project_tree, temp_root};
use super::super::super::test_support::{crate_facts, crate_input};
use super::super::check;
use guardrail3_domain_report::Severity;

#[test]
fn errors_on_invalid_semver_and_skips_non_publishable_crates() {
    let mut invalid = crate_facts("x");
    invalid.version_valid = false;
    invalid.version_string = Some("bad".to_owned());
    let invalid_input = crate_input(&invalid);
    let mut invalid_results = Vec::new();
    check(&invalid_input, &mut invalid_results);
    assert_eq!(invalid_results.len(), 1);
    assert_eq!(invalid_results[0].id, "RS-PUB-08");
    assert_eq!(invalid_results[0].severity, Severity::Error);
    assert!(!invalid_results[0].inventory);
    assert_eq!(
        invalid_results[0].file.as_deref(),
        Some("crates/example/Cargo.toml")
    );
    assert!(invalid_results[0].title.contains("invalid semver"));
    assert!(
        invalid_results[0]
            .message
            .contains("valid semver version or `version.workspace = true`")
    );

    let mut inherited_invalid = crate_facts("x");
    inherited_invalid.workspace_version = true;
    inherited_invalid.version_valid = false;
    inherited_invalid.version_string = None;
    let inherited_invalid_input = crate_input(&inherited_invalid);
    let mut inherited_invalid_results = Vec::new();
    check(&inherited_invalid_input, &mut inherited_invalid_results);
    assert_eq!(inherited_invalid_results.len(), 1);
    assert_eq!(inherited_invalid_results[0].id, "RS-PUB-08");
    assert_eq!(inherited_invalid_results[0].severity, Severity::Error);
    assert!(!inherited_invalid_results[0].inventory);
    assert_eq!(
        inherited_invalid_results[0].file.as_deref(),
        Some("crates/example/Cargo.toml")
    );
    assert!(
        inherited_invalid_results[0]
            .title
            .contains("invalid semver")
    );

    let mut non_publishable = crate_facts("x");
    non_publishable.publishable = false;
    non_publishable.version_valid = false;
    let non_publishable_input = crate_input(&non_publishable);
    let mut non_publishable_results = Vec::new();
    check(&non_publishable_input, &mut non_publishable_results);
    assert!(non_publishable_results.is_empty());
}

#[test]
fn errors_when_workspace_version_is_missing_or_invalid() {
    let missing_root = temp_root("release-workspace-version-missing");
    let missing_tree = project_tree(
        vec![
            ("", dir_entry(&["crates"], &["Cargo.toml"])),
            ("crates", dir_entry(&["pub"], &[])),
            ("crates/pub", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![
            (
                "Cargo.toml",
                r#"
[workspace]
members = ["crates/pub"]
resolver = "2"

[workspace.package]
description = "shared"
license = "MIT"
repository = "https://example.com/repo"
"#,
            ),
            (
                "crates/pub/Cargo.toml",
                r#"
[package]
name = "pub"
version.workspace = true
edition = "2024"
description.workspace = true
license.workspace = true
repository.workspace = true
readme = false
"#,
            ),
        ],
        missing_root,
    );
    let missing_results = run_family(&missing_tree, &StubToolChecker::new(true), false);
    assert!(missing_results.iter().any(|result| {
        result.id == "RS-PUB-08"
            && result.severity == Severity::Error
            && result.file.as_deref() == Some("crates/pub/Cargo.toml")
    }));

    let invalid_root = temp_root("release-workspace-version-invalid");
    let invalid_tree = project_tree(
        vec![
            ("", dir_entry(&["crates"], &["Cargo.toml"])),
            ("crates", dir_entry(&["pub"], &[])),
            ("crates/pub", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![
            (
                "Cargo.toml",
                r#"
[workspace]
members = ["crates/pub"]
resolver = "2"

[workspace.package]
version = "bad"
description = "shared"
license = "MIT"
repository = "https://example.com/repo"
"#,
            ),
            (
                "crates/pub/Cargo.toml",
                r#"
[package]
name = "pub"
version.workspace = true
edition = "2024"
description.workspace = true
license.workspace = true
repository.workspace = true
readme = false
"#,
            ),
        ],
        invalid_root,
    );
    let invalid_results = run_family(&invalid_tree, &StubToolChecker::new(true), false);
    assert!(invalid_results.iter().any(|result| {
        result.id == "RS-PUB-08"
            && result.severity == Severity::Error
            && result.file.as_deref() == Some("crates/pub/Cargo.toml")
    }));
}

#[test]
fn inventories_when_workspace_version_is_inherited_via_package_workspace_reference() {
    let root = temp_root("release-workspace-version-package-workspace");
    let tree = project_tree(
        vec![
            ("", dir_entry(&["ws", "packages"], &[])),
            ("ws", dir_entry(&[], &["Cargo.toml"])),
            ("packages", dir_entry(&["pub"], &[])),
            ("packages/pub", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![
            (
                "ws/Cargo.toml",
                r#"
[workspace]
members = ["../packages/pub"]
resolver = "2"

[workspace.package]
version = "1.2.3"
description = "shared"
license = "MIT"
repository = "https://example.com/repo"
"#,
            ),
            (
                "packages/pub/Cargo.toml",
                r#"
[package]
name = "pub"
workspace = "../../ws"
version.workspace = true
edition = "2024"
description.workspace = true
license.workspace = true
repository.workspace = true
readme = false
"#,
            ),
        ],
        root,
    );
    let results = run_family(&tree, &StubToolChecker::new(true), false);

    assert!(results.iter().any(|result| {
        result.id == "RS-PUB-08"
            && result.severity == Severity::Info
            && result.inventory
            && result.file.as_deref() == Some("packages/pub/Cargo.toml")
            && result.message.contains("`version.workspace = true`")
    }));
}

#[test]
fn errors_when_package_workspace_reference_does_not_include_crate() {
    let root = temp_root("release-workspace-version-package-workspace-orphan");
    let tree = project_tree(
        vec![
            ("", dir_entry(&["ws", "packages"], &[])),
            ("ws", dir_entry(&[], &["Cargo.toml"])),
            ("packages", dir_entry(&["member", "orphan"], &[])),
            ("packages/member", dir_entry(&[], &["Cargo.toml"])),
            ("packages/orphan", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![
            (
                "ws/Cargo.toml",
                r#"
[workspace]
members = ["../packages/member"]
resolver = "2"

[workspace.package]
version = "1.2.3"
description = "shared"
license = "MIT"
repository = "https://example.com/repo"
"#,
            ),
            (
                "packages/member/Cargo.toml",
                r#"
[package]
name = "member"
version = "0.1.0"
edition = "2024"
publish = false
"#,
            ),
            (
                "packages/orphan/Cargo.toml",
                r#"
[package]
name = "orphan"
workspace = "../../ws"
version.workspace = true
edition = "2024"
description.workspace = true
license.workspace = true
repository.workspace = true
readme = false
"#,
            ),
        ],
        root,
    );
    let results = run_family(&tree, &StubToolChecker::new(true), false);

    assert!(results.iter().any(|result| {
        result.id == "RS-PUB-08"
            && result.severity == Severity::Error
            && !result.inventory
            && result.file.as_deref() == Some("packages/orphan/Cargo.toml")
    }));
}
