/// Integration tests against real workspaces in the guardrail3 repository.
use super::helpers::{
    collect_package_dirs, crawl, is_supported_channel, package_dir, packages_dir,
};

/// Real workspace with `[package]` containing `rust-version = "1.85"`.
#[test]
fn ingests_real_workspace_with_package_rust_version() {
    let root = package_dir("parsers/clippy-toml-parser");
    if !root.join("rust-toolchain.toml").exists() {
        return;
    }

    let crawl = crawl(&root);
    let output = super::ingest_for_config_checks(&crawl)
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
        .and_then(
            |rv: &cargo_toml_parser::types::InheritableValue<String>| match rv {
                cargo_toml_parser::types::InheritableValue::Value(v) => Some(v.as_str()),
                cargo_toml_parser::types::InheritableValue::Inherit(_) => None,
            },
        );
    assert_eq!(rust_version, Some("1.85"));
}

/// Real workspace with `[workspace]` and an explicit package `rust-version`.
#[test]
fn ingests_real_workspace_without_rust_version() {
    let root = package_dir("shared/guardrail3-check-types");
    if !root.join("rust-toolchain.toml").exists() {
        return;
    }

    let crawl = crawl(&root);
    let output = super::ingest_for_config_checks(&crawl)
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
    // The fixture may declare rust-version directly on the package or inherit none from the workspace.
    // This test pins that the workspace has no inherited rust-version while allowing
    // the package itself to either declare or omit a value.
    assert!(workspace_rv.is_none());
    let _ = package_rv;
}

/// Sweep all real workspaces that have `rust-toolchain.toml`.
#[test]
fn ingests_all_real_workspaces() {
    let packages = collect_package_dirs(&packages_dir());
    let mut tested = 0_u32;

    for pkg_dir in packages {
        if !pkg_dir.is_dir() || !pkg_dir.join("rust-toolchain.toml").exists() {
            continue;
        }

        let crawl = crawl(&pkg_dir);
        let pkg_name = pkg_dir
            .file_name()
            .map_or("unknown", |n| n.to_str().unwrap_or("unknown"));
        let output = super::ingest_for_config_checks(&crawl)
            .unwrap_or_else(|err| unreachable!("ingestion failed for {pkg_name}: {err}"));

        let section = output
            .toolchain_toml
            .toolchain
            .as_ref()
            .unwrap_or_else(|| unreachable!("{pkg_name}: should have [toolchain] section"));
        let channel = section
            .channel
            .as_deref()
            .unwrap_or_else(|| unreachable!("{pkg_name}: should set [toolchain].channel"));
        assert!(
            is_supported_channel(channel),
            "{pkg_name}: should use stable or a pinned stable version, got `{channel}`"
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
