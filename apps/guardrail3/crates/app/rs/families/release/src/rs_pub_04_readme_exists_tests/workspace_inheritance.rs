use super::super::run_tree as check;
use super::super::{StubToolChecker, dir_entry, project_tree, temp_root};

#[test]
fn should_inventory_when_readme_is_inherited_from_workspace_package() {
    let root = temp_root("release-workspace-readme-inheritance");
    let tree = project_tree(
        vec![
            ("", dir_entry(&["crates"], &["Cargo.toml", "WORKSPACE.md"])),
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
version = "0.1.0"
description = "shared workspace description"
license = "MIT"
repository = "https://example.com/repo"
readme = "WORKSPACE.md"
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
readme.workspace = true
"#,
            ),
            (
                "WORKSPACE.md",
                "# Shared README\n\nWorkspace-owned readme.\n",
            ),
        ],
        root,
    );
    let results = check(&tree, &StubToolChecker::new(true), false);

    assert!(results.iter().any(|result| {
        result.id == "RS-PUB-04"
            && result.inventory
            && result.file.as_deref() == Some("WORKSPACE.md")
    }));
}

#[test]
fn should_resolve_workspace_readme_relative_to_workspace_root() {
    let root = temp_root("release-workspace-readme-relative-path");
    let tree = project_tree(
        vec![
            ("", dir_entry(&["ws"], &["README.md"])),
            ("ws", dir_entry(&["crates"], &["Cargo.toml"])),
            ("ws/crates", dir_entry(&["pub"], &[])),
            ("ws/crates/pub", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![
            (
                "ws/Cargo.toml",
                r#"
[workspace]
members = ["crates/pub"]
resolver = "2"

[workspace.package]
version = "0.1.0"
description = "shared workspace description"
license = "MIT"
repository = "https://example.com/repo"
readme = "../README.md"
"#,
            ),
            (
                "ws/crates/pub/Cargo.toml",
                r#"
[package]
name = "pub"
version.workspace = true
edition = "2024"
description.workspace = true
license.workspace = true
repository.workspace = true
readme.workspace = true
"#,
            ),
            (
                "README.md",
                "# Shared README\n\nWorkspace-relative readme.\n",
            ),
        ],
        root,
    );
    let results = check(&tree, &StubToolChecker::new(true), false);

    assert!(results.iter().any(|result| {
        result.id == "RS-PUB-04" && result.inventory && result.file.as_deref() == Some("README.md")
    }));
}

#[test]
fn should_warn_when_non_member_crate_tries_to_inherit_workspace_readme() {
    let root = temp_root("release-workspace-readme-orphan");
    let tree = project_tree(
        vec![
            ("", dir_entry(&["crates"], &["Cargo.toml", "WORKSPACE.md"])),
            ("crates", dir_entry(&["member", "orphan"], &[])),
            ("crates/member", dir_entry(&[], &["Cargo.toml"])),
            ("crates/orphan", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![
            (
                "Cargo.toml",
                r#"
[workspace]
members = ["crates/member"]
resolver = "2"

[workspace.package]
version = "0.1.0"
readme = "WORKSPACE.md"
"#,
            ),
            (
                "crates/member/Cargo.toml",
                r#"
[package]
name = "member"
version.workspace = true
edition = "2024"
publish = false
"#,
            ),
            (
                "crates/orphan/Cargo.toml",
                r#"
[package]
name = "orphan"
version = "0.1.0"
edition = "2024"
readme.workspace = true
"#,
            ),
            (
                "WORKSPACE.md",
                "# Shared README\n\nWorkspace-owned readme.\n",
            ),
        ],
        root,
    );
    let results = check(&tree, &StubToolChecker::new(true), false);

    assert!(results.iter().any(|result| {
        result.id == "RS-PUB-04"
            && result.severity == guardrail3_domain_report::Severity::Warn
            && !result.inventory
            && result.file.as_deref() == Some("crates/orphan/Cargo.toml")
    }));
}

#[test]
fn should_stay_quiet_when_readme_false_is_inherited_from_workspace_package() {
    let root = temp_root("release-workspace-readme-false-inheritance");
    let tree = project_tree(
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
version = "0.1.0"
readme = false
"#,
            ),
            (
                "crates/pub/Cargo.toml",
                r#"
[package]
name = "pub"
version.workspace = true
edition = "2024"
readme.workspace = true
"#,
            ),
        ],
        root,
    );
    let results = check(&tree, &StubToolChecker::new(true), false);

    assert!(
        results.iter().all(|result| result.id != "RS-PUB-04"),
        "workspace-inherited readme=false should suppress RS-PUB-04: {results:#?}"
    );
}
