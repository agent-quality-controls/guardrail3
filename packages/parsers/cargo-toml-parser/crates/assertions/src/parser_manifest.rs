#![allow(
    clippy::expect_used,
    clippy::missing_const_for_fn,
    clippy::missing_panics_doc,
    reason = "assertion helpers are reusable panic-based proof sites for test harnesses"
)]

use cargo_toml_parser_runtime::types::{CargoToml, Value};

pub fn assert_manifest_empty(manifest: &CargoToml) {
    assert!(
        manifest.cargo_features.is_empty(),
        "cargo_features should be empty"
    );
    assert_eq!(manifest.package, None, "package should be None");
    assert_eq!(manifest.project, None, "project should be None");
    assert_eq!(manifest.workspace, None, "workspace should be None");
    assert!(manifest.badges.is_empty(), "badges should be empty");
    assert!(
        manifest.dependencies.is_empty(),
        "dependencies should be empty"
    );
    assert!(manifest.target.is_empty(), "target should be empty");
    assert!(manifest.profile.is_empty(), "profile should be empty");
    assert!(manifest.extra.is_empty(), "extra should be empty");
}

pub fn assert_top_level_extra_string(manifest: &CargoToml, key: &str, expected: &str) {
    assert_eq!(
        manifest.extra.get(key).and_then(Value::as_str),
        Some(expected),
        "top-level extra string mismatch",
    );
}

pub fn assert_string_build_and_readme(manifest: &CargoToml) {
    use cargo_toml_parser_runtime::types::{InheritableValue, PackageBuildValue, StringOrBool};

    let package = manifest.package.as_ref();
    assert!(package.is_some(), "package should exist");
    let Some(package) = package else {
        return;
    };
    assert_eq!(
        package.build,
        Some(PackageBuildValue::SingleScript("build.rs".to_owned())),
        "package.build mismatch",
    );
    assert_eq!(
        package.readme,
        Some(InheritableValue::Value(StringOrBool::String(
            "README.md".to_owned(),
        ))),
        "package.readme mismatch",
    );
}

pub fn assert_package_name(manifest: &CargoToml, expected: &str) {
    assert_eq!(
        manifest
            .package
            .as_ref()
            .and_then(|package| package.name.as_deref()),
        Some(expected),
        "package.name mismatch",
    );
}

pub fn assert_parse_error(err: impl std::fmt::Display) {
    let msg = err.to_string();
    assert!(
        msg.contains("invalid Cargo.toml"),
        "expected Cargo.toml parse error prefix, got: {msg}",
    );
}
