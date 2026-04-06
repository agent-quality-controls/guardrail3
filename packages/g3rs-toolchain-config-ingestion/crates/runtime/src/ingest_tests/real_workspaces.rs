/// Integration tests against real workspaces in the guardrail3 repository.

use std::path::{Path, PathBuf};

/// Resolve the guardrail3 packages directory from the runtime crate's manifest.
fn packages_dir() -> PathBuf {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    manifest
        .parent()
        .and_then(Path::parent)
        .and_then(Path::parent)
        .expect("should resolve packages/ directory from CARGO_MANIFEST_DIR")
        .to_path_buf()
}

fn crawl(root: &Path) -> g3rs_workspace_crawl::G3RsWorkspaceCrawl {
    g3rs_workspace_crawl::crawl(root).expect("crawl should succeed on a real workspace")
}

/// Real workspace with `[package]` containing `rust-version = "1.85"`.
#[test]
fn ingests_real_workspace_with_package_rust_version() {
    let root = packages_dir().join("clippy-toml-parser");
    if !root.join("rust-toolchain.toml").exists() {
        return;
    }

    let crawl = crawl(&root);
    let output = crate::ingest(&crawl)
        .expect("ingestion should succeed on clippy-toml-parser");

    assert_eq!(output.toolchain_rel_path, "rust-toolchain.toml");
    let section = output
        .toolchain_toml
        .toolchain
        .as_ref()
        .expect("should have a [toolchain] section");
    assert_eq!(section.channel.as_deref(), Some("stable"));
    assert_eq!(
        section.components,
        vec!["clippy".to_owned(), "rustfmt".to_owned()],
    );

    let cargo = output
        .cargo_toml
        .as_ref()
        .expect("cargo_toml should be Some");
    assert_eq!(output.cargo_rel_path.as_deref(), Some("Cargo.toml"));

    let rust_version = cargo
        .package
        .as_ref()
        .and_then(|pkg| pkg.rust_version.as_ref())
        .and_then(|rv| match rv {
            cargo_toml_parser::InheritableValue::Value(v) => Some(v.as_str()),
            cargo_toml_parser::InheritableValue::Inherit(_) => None,
        });
    assert_eq!(rust_version, Some("1.85"));
}

/// Real workspace with `[workspace]` and no `rust-version` anywhere.
#[test]
fn ingests_real_workspace_without_rust_version() {
    let root = packages_dir().join("guardrail3-check-types");
    if !root.join("rust-toolchain.toml").exists() {
        return;
    }

    let crawl = crawl(&root);
    let output = crate::ingest(&crawl)
        .expect("ingestion should succeed on guardrail3-check-types");

    let section = output
        .toolchain_toml
        .toolchain
        .as_ref()
        .expect("should have a [toolchain] section");
    assert_eq!(section.channel.as_deref(), Some("stable"));

    let cargo = output
        .cargo_toml
        .as_ref()
        .expect("cargo_toml should be Some");
    let workspace_rv = cargo
        .workspace
        .as_ref()
        .and_then(|ws| ws.package.as_ref())
        .and_then(|pkg| pkg.rust_version.as_deref());
    let package_rv = cargo
        .package
        .as_ref()
        .and_then(|pkg| pkg.rust_version.as_ref());
    assert!(workspace_rv.is_none());
    assert!(package_rv.is_none());
}

/// Sweep all real workspaces that have `rust-toolchain.toml`.
#[test]
fn ingests_all_real_workspaces() {
    let packages = packages_dir();
    let mut tested = 0_u32;

    let entries = std::fs::read_dir(&packages)
        .expect("should be able to list the packages directory");
    for entry in entries {
        let entry = entry.expect("should read directory entry");
        let pkg_dir = entry.path();
        if !pkg_dir.is_dir() || !pkg_dir.join("rust-toolchain.toml").exists() {
            continue;
        }

        let crawl = crawl(&pkg_dir);
        let pkg_name = pkg_dir
            .file_name()
            .map_or("unknown", |n| n.to_str().unwrap_or("unknown"));
        let output = crate::ingest(&crawl)
            .unwrap_or_else(|err| panic!("ingestion failed for {pkg_name}: {err}"));

        let section = output
            .toolchain_toml
            .toolchain
            .as_ref()
            .unwrap_or_else(|| panic!("{pkg_name}: should have [toolchain] section"));
        assert_eq!(
            section.channel.as_deref(),
            Some("stable"),
            "{pkg_name}: should use channel = stable"
        );
        assert!(
            section.components.contains(&"clippy".to_owned()),
            "{pkg_name}: should include clippy"
        );
        assert!(
            section.components.contains(&"rustfmt".to_owned()),
            "{pkg_name}: should include rustfmt"
        );
        assert!(
            output.cargo_toml.is_some(),
            "{pkg_name}: should have Cargo.toml"
        );

        tested = tested.saturating_add(1);
    }

    assert!(
        tested >= 5,
        "should have tested at least 5 real workspaces, only found {tested}"
    );
}
